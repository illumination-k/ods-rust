use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct Node<T> {
    x: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(x: T) -> Self {
        Self {
            x,
            next: None,
            prev: None,
        }
    }

    pub fn into_element(self: Box<Self>) -> T {
        self.x
    }
}

pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a DLList<T>,
}

impl<'a, T> Cursor<'a, T> {
    pub fn move_next(&mut self) {
        match self.current.take() {
            Some(cur) => unsafe {
                self.current = cur.as_ref().next;
                self.index += 1;
            },
            None => {
                self.current = self.list.head;
                self.index = 0;
            }
        }
    }

    pub fn move_prev(&mut self) {
        match self.current.take() {
            Some(cur) => unsafe {
                self.current = cur.as_ref().prev;
                self.index -= 1;
            },
            None => {
                self.current = self.list.tail;
                self.index = self.index.checked_sub(1).unwrap_or_else(|| self.list.n);
            }
        }
    }

    pub fn current(&self) -> Option<&'a T> {
        unsafe { self.current.map(|cur| &(*cur.as_ptr()).x) }
    }
}

#[derive(Debug)]
pub struct DLList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    n: usize,
    marker: PhantomData<Box<Node<T>>>,
}

// private methods
impl<T> DLList<T> {
    fn get_node(&self, index: usize) -> Option<NonNull<Node<T>>> {
        if index < self.n / 2 {
            let mut cursor = Cursor {
                index: 0,
                current: self.head,
                list: &self,
            };

            for _ in 0..index {
                cursor.move_next();
            }

            cursor.current
        } else {
            let mut cursor = Cursor {
                index: self.n,
                current: self.tail,
                list: &self,
            };

            for _ in index..self.n - 1 {
                cursor.move_prev();
            }

            cursor.current
        }
    }

    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.n += 1;
        }
    }

    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = None;
            node.prev = self.tail;

            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.n += 1;
        }
    }

    fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
        let node = unsafe { node.as_mut() };

        match node.prev {
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
            None => self.head = node.next,
        };

        match node.next {
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
            None => self.tail = node.prev,
        }

        self.n -= 1;
    }

    fn splice_node(
        &mut self,
        existing_prev: Option<NonNull<Node<T>>>,
        existing_next: Option<NonNull<Node<T>>>,
        mut splice_node: NonNull<Node<T>>,
    ) {
        if let Some(mut existing_prev) = existing_prev {
            unsafe {
                existing_prev.as_mut().next = Some(splice_node);
            }
        } else {
            self.head = Some(splice_node)
        }

        if let Some(mut existing_next) = existing_next {
            unsafe {
                existing_next.as_mut().prev = Some(splice_node);
            }
        } else {
            self.tail = Some(splice_node)
        }

        unsafe {
            splice_node.as_mut().prev = existing_prev;
            splice_node.as_mut().next = existing_next;
        }

        self.n += 1;
    }
}

impl<T> DLList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            n: 0,
            marker: PhantomData,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        unsafe { self.get_node(index).map(|n| &(*n.as_ptr()).x) }
    }

    pub fn set(&mut self, x: T, index: usize) {
        unsafe {
            self.get_node(index).map(|mut n| n.as_mut().x = x);
        }
    }

    pub fn push_front(&mut self, x: T) {
        let node = Box::new(Node::new(x));
        self.push_front_node(node);
    }

    pub fn push_back(&mut self, x: T) {
        let node = Box::new(Node::new(x));
        self.push_back_node(node);
    }

    pub fn add(&mut self, x: T, index: usize) {
        let current = self.get_node(index);
        unsafe {
            let spliced_node = Box::leak(Box::new(Node::new(x))).into();
            let node_prev = match current {
                None => self.tail,
                Some(node) => node.as_ref().prev,
            };
            self.splice_node(node_prev, current, spliced_node);
        }

        // u.next -> w
        // u.prev -> w.prev
    }

    pub fn remove(&mut self, index: usize) -> Option<&T> {
        let node = self.get_node(index);

        if let Some(node) = node {
            self.unlink_node(node);
        }

        unsafe { node.map(|n| &(*n.as_ptr()).x) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let mut l = DLList::new();

        let slice = &[0, 1, 2, 3, 4];
        for &i in slice {
            l.push_back(i);
        }

        for i in 0..l.size() {
            assert_eq!(l.get(i), Some(&i));
            println!("i: {}, val: {:?}", i, &l.get(i))
        }

        assert_eq!(l.size(), slice.len());

        println!("========");

        l.add(111, 3);
        assert_eq!(l.size(), slice.len() + 1);
        assert_eq!(l.get(3), Some(&111));
        assert_eq!(l.get(4), Some(&3));

        l.set(444, 4);

        assert_eq!(l.get(4), Some(&444));
        l.remove(1);
        assert_eq!(l.size(), slice.len());
        assert_eq!(l.get(1), Some(&2));
        for i in 0..l.size() {
            println!("i: {}, val: {:?}", i, &l.get(i))
        }
    }
}

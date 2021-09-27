pub mod bdeque;

use std::{
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use bdeque::BDeque;

pub struct Node<T> {
    d: BDeque<T>,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T: Clone + Default> Node<T> {
    pub fn new(b: usize) -> Self {
        Self {d: BDeque::new(b+1),
        prev: None,
        next: None,}
    }
}

impl<T> Index<usize> for Node<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.d[index]
    }
}

impl<T> IndexMut<usize> for Node<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.d[index]
    }
}

#[derive(Debug)]
pub struct Cursor<'a, T: 'a> {
    index: usize,
    j: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a SEList<T>,
}

impl<'a, T:> Cursor<'a, T> {
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
                self.index = self.index.checked_sub(1).unwrap_or_else(|| self.list.node_size);
            }
        }
    }

    pub fn node_ptr(&self) -> Option<NonNull<Node<T>>> {
        self.current.clone()
    }

    pub fn size(&self) -> Option<usize> {
        unsafe {
            self.current.map(|n| n.as_ref().d.size())
        }
    }

    pub fn current(&self) -> Option<&'a T> {
        unsafe { self.current.map(|cur| &(*cur.as_ptr()).d[self.j]) }
    }

    pub fn current_mut(&mut self) -> Option<&'a mut T> {
        unsafe { self.current.map(|cur| &mut (*cur.as_ptr()).d[self.j])}
    }
}

#[derive(Debug)]
pub struct SEList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    n: usize,
    b: usize,
    node_size: usize,
}

impl<T: Clone + Default> SEList<T> {
    fn get_location(&self, mut index: usize) -> Option<Cursor<T>> {
        if index >= self.n {
            return None
        }

        if index < self.n {
            let mut cursor = Cursor {
                index: 0,
                j: 0,
                current: self.head,
                list: &self,
            };

            loop {
                if let Some(s) = cursor.size() {
                    if index < s {
                        cursor.j = index;
                        return Some(cursor)
                    }

                    index -= s;
                    cursor.move_next();
                } else {
                    return None
                }
            }

        } else {
            let mut cursor = Cursor {
                index: self.node_size,
                j: 0,
                current: self.tail,
                list: &self,
            };

            let mut idx = self.n;
            loop {
                if let Some(s) = cursor.size() {
                    if index >= idx {
                        cursor.j = idx;
                        return Some(cursor)
                    }

                    cursor.move_prev();
                    idx -= s;
                } else {
                    return None
                }
            }
        }
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

        self.node_size += 1;
    }

    fn push_back_node(&mut self) {
        let mut node = Box::new(Node::new(self.b));
        node.next = None;
        node.prev = self.tail;

        let node = Some(Box::leak(node).into());

        unsafe {
            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
        }

        self.tail = node;

        // あくまでノードを最後に足すだけ
        self.node_size += 1;
    }

    fn spread(&mut self, mut cur: Cursor<T>) {
        for _ in 0..self.b {
            cur.move_next();
        }
    }
}

impl<T: Clone + Default> SEList<T> {
    pub fn new(b: usize) -> Self {
        Self {
            head: None,
            tail: None,
            b: b,
            n: 0,
            node_size: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let cur = self.get_location(index);

        match cur {
            None => None,
            Some(c) => c.current(),
        }
    }

    pub fn set(&self, index: usize, x: T) {
        let cur = self.get_location(index);
        match cur {
            None => {},
            Some(mut c) => { c.current_mut().map(|t| *t = x); },
        };
    }

    pub fn push_back(&mut self, x: T) {
        if let Some(tail) = self.tail {
            unsafe {
                if tail.as_ref().d.size() == self.b + 1 {
                    self.push_back_node();
                }
            }
        } else {
            self.push_back_node();
        }

        // tail mut not be None
        self.n += 1;
        if let Some(mut tail) = self.tail {
            unsafe {
                tail.as_mut().d.push_back(x);
            }
        }
    }

    pub fn add(&mut self, index: usize, x: T) {
        if index == self.n {
            self.push_back(x);
            return;
        }

        let cursor = self.get_location(index);
        
        if let Some(mut cursor) = cursor {
            let mut u = cursor.current.clone();
            let mut r = 0;
            while r < self.b && cursor.current.is_some() && cursor.size().unwrap() == self.b + 1 {
                cursor.move_next();
                r += 1;
            }

            if r == self.b {
                // b + 1要素を含むブロックがb個
                // spread
            }

            // 末尾まで到達？
            if cursor.current == self.tail {
                self.push_back_node();
            }

            // 要素をシフト
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cursor() {
        let mut l = SEList::new(4);
        for i in 0..10 {
            l.push_back(i)
        }

        let mut cur = l.get_location(0).unwrap();
        let u = cur.node_ptr();
        cur.move_next();
        cur.move_prev();
        assert_eq!(u, cur.node_ptr())
    }
    #[test]
    fn test_1() {
        let mut l = SEList::new(4);
        for i in 0..10 {
            l.push_back(i)
        }

        l.set(1, 1000);

        for i in 0..=10 {
            dbg!(&l.get(i));
        }
    }
}

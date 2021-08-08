use std::cell::RefCell;
use std::rc::Rc;

pub type List<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    x: T,
    next: Option<List<T>>,
}

impl<T> Node<T> {
    pub fn new(x: T, next: Option<List<T>>) -> List<T> {
        Rc::new(RefCell::new(Self { x, next }))
    }
}

#[derive(Debug, Clone)]
pub struct SLList<T> {
    head: Option<List<T>>,
    tail: Option<List<T>>,
    n: usize,
}

impl<T> SLList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            n: 0,
        }
    }
    pub fn size(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, x: T) {
        let new_node = Node::new(x, None);

        match self.head.take() {
            Some(prev) => new_node.borrow_mut().next = Some(Rc::clone(&prev)),
            None => self.tail = Some(Rc::clone(&new_node)),
        };

        self.n += 1;
        self.head = Some(new_node);
    }

    pub fn add(&mut self, x: T) {
        let new_node = Node::new(x, None);

        match self.tail.take() {
            Some(prev) => prev.borrow_mut().next = Some(Rc::clone(&new_node)),
            None => self.head = Some(Rc::clone(&new_node)),
        }

        self.n += 1;
        self.tail = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let x = self.head.take().map(|prev| {
            if let Some(next) = prev.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            Rc::try_unwrap(prev).ok().unwrap().into_inner().x
        });
        self.n -= 1;

        x
    }
}

#[cfg(test)]
mod test_sllist {
    use super::*;

    #[test]
    fn test_sllist_1() {
        let mut sllist = SLList::new();
        sllist.push('a');
        sllist.push('b');
        sllist.add('c');
        dbg!(&sllist);
        assert_eq!(sllist.pop(), Some('b'));
        assert_eq!(sllist.pop(), Some('a'));
        assert_eq!(sllist.pop(), Some('c'));
        dbg!(&sllist);
    }
}

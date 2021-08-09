pub mod bdeque;

use std::{
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use bdeque::BDeque;

struct Node<T> {
    d: BDeque<T>,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {}

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

pub struct SEList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    n: usize,
}

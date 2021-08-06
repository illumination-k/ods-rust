use crate::array::allocate_heap;
use crate::array::stack::ArrayStack;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

fn i2b(i: usize) -> usize {
    let db = (-3. + ((9 + 8 * i) as f64).sqrt()) / 2.;
    db.ceil() as usize
}

#[derive(Debug)]
pub struct RootishArrayStack<T> {
    blocks: ArrayStack<Box<[T]>>,
    n: usize,
}

impl<T> RootishArrayStack<T> {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn size(&self) -> usize {
        self.n
    }
}

impl<T> RootishArrayStack<T>
where
    T: Clone + Default + Debug,
{
    pub fn new() -> Self {
        Self {
            blocks: ArrayStack::new(0),
            n: 0,
        }
    }
    pub fn grow(&mut self) {
        self.blocks.add(self.size(), allocate_heap(self.size() + 1));
    }

    pub fn shrink(&mut self) {
        let n = self.size();
        let mut r = self.blocks.size();
        while r > 0 && (r - 2) * (r - 1) / 2 >= n {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }

    pub fn add(&mut self, i: usize, x: T) {
        let r = self.blocks.size();
        let n = self.size();

        if r * (r + 1) / 2 < n + 1 {
            self.grow();
        }

        self.n += 1;
        for j in (i + 1..self.size()).rev() {
            let y = self[j - 1].clone();
            self[j] = y;
        }

        println!("ok: {:?}", x);

        self[i] = x;
    }

    pub fn remove(&mut self, i: usize) -> T {
        let x = self[i].clone();

        for j in i..self.size() - 1 {
            let y = self[j + 1].clone();
            self[j] = y;
        }

        self.n -= 1;
        let r = self.blocks.size();
        if (r - 2) * (r - 1) / 2 >= self.size() {
            self.shrink()
        }
        x
    }
}

impl<T> Index<usize> for RootishArrayStack<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let b = i2b(index);
        let j = index - b * (b + 1) / 2;
        &self.blocks[b][j]
    }
}

impl<T> IndexMut<usize> for RootishArrayStack<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let b = i2b(index);
        let j = index - b * (b + 1) / 2;
        &mut self.blocks[b][j]
    }
}

#[cfg(test)]
mod test_rootish_array_stack {
    use super::*;

    #[test]
    fn test_1() {
        let mut ras = RootishArrayStack::new();
        ras.add(0, 'a');
        ras.add(0, 'b');
        ras.add(1, 'c');
        ras.add(0, 'd');
        ras.remove(0);
        dbg!(&ras);
    }
}

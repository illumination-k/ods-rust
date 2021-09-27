use std::ops::{Index, IndexMut};

use crate::array::allocate_heap;

#[derive(Debug, Clone)]
pub struct BDeque<T> {
    a: Box<[T]>,
    n: usize,
    j: usize,
}

impl<T> BDeque<T>
where
    T: Default + Clone,
{
    pub fn new(b: usize) -> Self {
        Self {
            a: allocate_heap(b + 1),
            n: 0,
            j: 0,
        }
    }

    pub fn add(&mut self, index: usize, x: T) {
        let n = self.size();

        if index < n / 2 {
            self.j = if self.j == 0 {
                self.a.len() - 1
            } else {
                self.j - 1
            };

            for k in 0..index {
                self.a[self.mod_index(k)] = self.a[self.mod_index(k + 1)].clone();
            }
        } else {
            for k in (index + 1..=n).rev() {
                dbg!(&self.mod_index(k));
                self.a[self.mod_index(k)] = self.a[self.mod_index(k - 1)].clone();
            }
        }

        self.a[self.mod_index(index)] = x;
        self.n += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        let x = self.a[self.mod_index(index)].clone();

        if index < self.size() / 2 {
            for k in (1..=index).rev() {
                self.a[self.mod_index(k)] = self.a[self.mod_index(k - 1)].clone();
            }

            self.j = (self.j + 1) % self.a.len();
        } else {
            for k in index..self.size() - 1 {
                self.a[self.mod_index(k)] = self.a[self.mod_index(k + 1)].clone();
            }
        }

        x
    }

    pub fn push_back(&mut self, x: T) {
        self.add(self.size(), x)
    }
}

impl<T> BDeque<T> {
    pub fn size(&self) -> usize {
        self.n
    }

    pub fn mod_index(&self, index: usize) -> usize {
        (self.j + index) % self.a.len()
    }
}

impl<T> Index<usize> for BDeque<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.a[self.mod_index(index)]
    }
}

impl<T> IndexMut<usize> for BDeque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.a[self.mod_index(index)]
    }
}

#[cfg(test)]
mod test_array_bq {
    use super::*;

    #[test]
    fn test_array_dequeue_1() {
        let mut dq = BDeque::new(10);
        for c in "abcdef".chars() {
            dq.add(0, c);
        }
        dbg!(&dq);

        for c in "abcdef".chars().rev() {
            let x = dq.remove(0);
            dbg!(&x);
            assert_eq!(x, c);
        }
    }
}

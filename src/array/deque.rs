use crate::array::allocate_heap;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct ArrayDeque<T> {
    a: Box<[T]>,
    n: usize,
    j: usize,
}

impl<T> ArrayDeque<T>
where
    T: Default + Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            a: allocate_heap(n),
            n: 0,
            j: 0,
        }
    }

    pub fn resize(&mut self) {
        let new_length = std::cmp::max(2 * self.size(), 1);
        let mut b = allocate_heap(new_length);
        for k in 0..self.size() {
            b[k] = self.a[(self.j + k) % self.len()].clone();
        }

        let _old_a = std::mem::replace(&mut self.a, b);
        self.j = 0;
    }

    pub fn add(&mut self, i: usize, x: T) {
        let n = self.size();
        if n + 1 >= self.len() {
            self.resize();
        }

        if i < n / 2 {
            // 要素数の中央より小さいとき、左寄せa[0] - a[i - 1]
            // addされるのでjを左寄せ、0なら最後に
            self.j = if self.j == 0 {
                self.len() - 1
            } else {
                self.j - 1
            };

            for k in 0..i {
                self.a[self.mod_index(k)] = self.a[self.mod_index(k + 1)].clone();
            }
        } else {
            // 要素数の中央より大きいとき、右寄せa[i] - a[n]
            for k in (i + 1..self.n).rev() {
                self.a[self.mod_index(k)] = self.a[self.mod_index(k - 1)].clone();
            }
        }

        self.a[(self.j + i) % self.len()] = x;
        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> T {
        // addとは逆の動作をすれば良い
        let x = self.a[self.mod_index(i)].clone();

        if i < self.size() / 2 {
            for k in (1..=i).rev() {
                self.a[self.mod_index(k)] = self.a[self.mod_index(k - 1)].clone();
            }
            // この場合は初期位置がずれる
            self.j = (self.j + 1) % self.len();
        } else {
            for k in i..self.size() - 1 {
                self.a[self.mod_index(k)] = self.a[self.mod_index(k + 1)].clone();
            }
        }

        // 要素数をへらす
        self.n -= 1;

        if self.len() >= 3 * self.size() {
            self.resize()
        }

        x
    }
}

impl<T> ArrayDeque<T> {
    pub fn len(&self) -> usize {
        self.a.len()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn mod_index(&self, i: usize) -> usize {
        (self.j + i) % self.len()
    }
}

impl<T> Index<usize> for ArrayDeque<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let idx = (self.j + index) % self.len();
        &self.a[idx]
    }
}

impl<T> IndexMut<usize> for ArrayDeque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let idx = (self.j + index) % self.len();
        &mut self.a[idx]
    }
}

#[cfg(test)]
mod test_array_deque {
    use super::*;

    #[test]
    fn test_array_dequeue_1() {
        let mut dq = ArrayDeque::new(1);
        dq.add(0, 'a');
        dq.add(1, 'b');
        dq.add(0, 'c');
        let elem = dq.remove(0);
        assert_eq!(elem, 'c');
        dq.add(2, 'd');
        dq.add(3, 'e');
        let elem = dq.remove(3);
        assert_eq!(elem, 'e');
        let elem = dq.remove(1);
        assert_eq!(elem, 'b');
        dbg!(&dq);
    }
}

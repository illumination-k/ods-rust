use crate::array::allocate_heap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct ArrayStack<T> {
    a: Box<[T]>,
    n: usize,
}

impl<T> ArrayStack<T>
where
    T: Default + Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            a: allocate_heap(n),
            n: 0,
        }
    }

    /// 内部要素の数が多すぎたとき、少なすぎたときにメモリ上に確保する配列長を変更する
    pub fn resize(&mut self) {
        let new_length = std::cmp::max(self.size() * 2, 1);
        let mut b = allocate_heap(new_length);
        for i in 0..self.size() {
            b[i] = self[i].clone();
        }

        let _old_a = std::mem::replace(&mut self.a, b);
    }

    pub fn add(&mut self, i: usize, x: T) {
        let n = self.size();
        if n + 1 >= self.len() {
            self.resize();
        }

        if i >= n {
            self[n] = x;
        } else {
            // n番目に代入して、挿入した位置をrotate rightでずらす
            self.a[n] = x;
            self.a[i..=n].rotate_right(1);
        }

        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> T {
        let x = self.a[i].clone();
        let n = self.size();
        if i < n {
            // rotate leftでずらす
            self.a[i..n].rotate_left(1);
            self.n -= 1;

            // defaultでn番目を埋めたほうがいい？実装上はまあ必要ない

            if self.len() >= 3 * n {
                self.resize();
            }
        }
        x
    }
}

impl<T> ArrayStack<T> {
    /// 配列長
    pub fn len(&self) -> usize {
        self.a.len()
    }
    /// 内部要素の数
    pub fn size(&self) -> usize {
        self.n
    }

    pub fn size_mut(&mut self) -> &mut usize {
        &mut self.n
    }
}

// get and setの実装はIndexで十分かな
// assert index < self.size()があったほうがいい可能性

impl<T> Index<usize> for ArrayStack<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.a[index]
    }
}

impl<T> IndexMut<usize> for ArrayStack<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.a[index]
    }
}

#[cfg(test)]
mod test_arraystack {
    use super::*;

    #[test]
    fn test_index() {
        let mut arr = ArrayStack {
            a: Box::new([1, 2, 3]),
            n: 3,
        };

        assert_eq!(arr[1], 2);
        arr[1] = 3;
        assert_eq!(arr[1], 3);
    }

    #[test]
    fn test_resize() {
        let mut arr: ArrayStack<usize> = ArrayStack::new(4);
        arr.add(0, 2);
        assert_eq!(arr[0], 2);
        arr.add(0, 3);
        assert_eq!(arr[0], 3);
        arr.add(1, 4);
        assert_eq!(arr[1], 4);
        arr.remove(1);
        assert_eq!(arr[1], 2);
        arr.add(1, 3);
        assert_eq!(arr[1], 3);
        assert_eq!(arr[2], 2);
        arr.add(0, 1);
        dbg!(&arr);
    }
}

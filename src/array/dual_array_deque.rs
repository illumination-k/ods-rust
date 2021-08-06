use crate::array::stack::ArrayStack;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct DualArrayDeque<T> {
    // [0, front.size())の要素を逆順に入れる
    front: ArrayStack<T>,

    // [front.size(), self.size())の要素を順番に入れる
    back: ArrayStack<T>,
}

impl<T> DualArrayDeque<T>
where
    T: Clone + Default,
{
    pub fn new() -> Self {
        Self {
            front: ArrayStack::new(0),
            back: ArrayStack::new(0),
        }
    }
    pub fn add(&mut self, i: usize, x: T) {
        let f_n = self.front.size();
        if i < f_n {
            self.front.add(f_n - i, x);
        } else {
            self.back.add(i - f_n, x);
        }

        self.balance();
    }

    pub fn remove(&mut self, i: usize) -> T {
        let f_n = self.front.size();
        let x = if i < f_n {
            self.front.remove(f_n - i - 1)
        } else {
            self.back.remove(i - f_n)
        };

        self.balance();
        x
    }

    fn balance(&mut self) {
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let n = self.front.size() + self.back.size();
            let nf = n / 2;

            let mut new_front = ArrayStack::new(std::cmp::max(2 * nf, 1));
            for i in 0..nf {
                new_front[nf - i - 1] = self[i].clone();
            }

            *new_front.size_mut() = nf;

            let nb = n - nf;
            let mut new_back = ArrayStack::new(std::cmp::max(2 * nb, 1));

            for i in 0..nb {
                new_back[i] = self[nf + i].clone();
            }

            *new_back.size_mut() = nb;

            self.front = new_front;
            self.back = new_back;
        }
    }
}

impl<T> DualArrayDeque<T> {
    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }
    pub fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }
}

impl<T> Index<usize> for DualArrayDeque<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.front.size() {
            &self.front[self.front.size() - index - 1]
        } else {
            &self.back[index - self.front.size()]
        }
    }
}

impl<T> IndexMut<usize> for DualArrayDeque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let front_n = self.front.size();
        if index < front_n {
            &mut self.front[front_n - index - 1]
        } else {
            &mut self.back[index - front_n]
        }
    }
}

#[cfg(test)]
mod test {
    use super::DualArrayDeque;

    #[test]
    fn test_dualarraydeque() {
        let mut dad = DualArrayDeque::new();
        dad.add(0, 'a');
        dad.add(1, 'b');
        dad.add(2, 'c');
        dad.add(3, 'd');

        assert_eq!(dad[0], 'a');
        assert_eq!(dad[1], 'b');
        assert_eq!(dad[2], 'c');
        assert_eq!(dad[3], 'd');
        dad.add(3, 'x');
        dad.add(4, 'y');

        let elem = dad.remove(0);
        assert_eq!(elem, 'a');
        assert_eq!(dad[0], 'b');
        assert_eq!(dad[1], 'c');
        assert_eq!(dad[2], 'x');
        assert_eq!(dad[3], 'y');
        assert_eq!(dad[4], 'd');
        dbg!(&dad);
    }
}

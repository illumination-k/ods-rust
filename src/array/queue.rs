use crate::array::allocate_heap;

/// a: 本体  
/// n: 要素数  
/// j: スタート位置
#[derive(Debug)]
pub struct ArrayQueue<T> {
    a: Box<[T]>,
    n: usize,
    j: usize,
}

impl<T> ArrayQueue<T>
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

    pub fn len(&self) -> usize {
        self.a.len()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn add(&mut self, x: T) -> bool {
        let n = self.size();
        if n + 1 >= self.len() {
            self.resize();
        }
        self.a[(self.j + n) % self.len()] = x;
        self.n += 1;
        true
    }

    pub fn remove(&mut self) -> T {
        // 返り値を保存
        let x = self.a[self.j].clone();
        self.j = (self.j + 1) % self.len();

        // 要素数をへらす
        self.n -= 1;

        if self.len() >= 3 * self.size() {
            self.resize()
        }

        x
    }
}

#[cfg(test)]
mod test_array_queue {
    use super::*;

    #[test]
    fn test_array_queue_1() {
        let mut q = ArrayQueue::new(1);
        q.add('a');
        q.add('b');
        let elem = q.remove();
        assert_eq!(elem, 'a');
        q.add('c');
        let elem = q.remove();
        assert_eq!(elem, 'b');
        dbg!(&q);
    }
}

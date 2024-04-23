use crate::DoubleQueue;

#[derive(Debug)]
pub struct VecDoubleQueue<T> {
    data: Vec<T>,
    front: usize,
    size: usize,
    capacity: usize,
    capacity_ratio: usize,
}

impl<T: Copy + Default> VecDoubleQueue<T> {
    pub fn new() -> Self {
        Self {
            data: vec![Default::default(); 3],
            size: 0,
            front: 0,
            capacity: 3,
            capacity_ratio: 2,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn index(&self, index: i32) -> usize {
        ((index + self.capacity as i32) % self.capacity as i32) as usize
    }

    pub fn extend_capacity(&mut self) {
        if self.size == self.capacity {
            let new_capacity = self.capacity * self.capacity_ratio;
            let mut data = vec![Default::default(); new_capacity];

            for (i, v) in self.data.to_vec().into_iter().enumerate() {
                data[new_capacity - self.size + i] = v;
            }

            self.data = data;
            self.front = new_capacity - self.size;
            self.capacity = new_capacity;
        }
    }
}

impl<T: Copy + Default> DoubleQueue for VecDoubleQueue<T> {
    type Item = T;

    fn push_first(&mut self, value: T) {
        self.extend_capacity();

        self.front = self.index(self.front as i32 - 1);

        self.data[self.front] = value;

        self.size += 1;
    }

    fn push_last(&mut self, value: T) {
        self.extend_capacity();

        let rear = self.index((self.front + self.size) as i32);

        self.data[rear] = value;

        self.size += 1;
    }

    fn peek_first(&self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let value = self.data[self.front];

        Some(value)
    }

    fn peek_last(&self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let rear = self.index(self.front as i32 + self.size as i32 - 1);

        let value = self.data[rear];

        Some(value)
    }

    fn pop_first(&mut self) -> Option<T> {
        self.peek_first().and_then(|x| {
            self.front = self.index(self.front as i32 + 1);

            self.size -= 1;

            Some(x)
        })
    }

    fn pop_last(&mut self) -> Option<T> {
        self.peek_last().and_then(|x| {
            self.size -= 1;

            Some(x)
        })
    }

    fn to_vec(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.capacity);

        let mut front = self.front;
        let mut size = self.size;

        while size > 0 {
            res.push(self.data[front]);

            front = self.index(front as i32 + 1);

            size -= 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_double_queue() {
        let mut queue = VecDoubleQueue::new();

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);

        queue.push_first(1);
        queue.push_first(2);
        queue.push_first(3);
        queue.push_last(4);
        queue.push_last(5);
        queue.push_last(6);

        assert_eq!(queue.size(), 6);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.pop_first(), Some(3));
        assert_eq!(queue.pop_first(), Some(2));
        assert_eq!(queue.pop_first(), Some(1));

        assert_eq!(queue.pop_last(), Some(6));
        assert_eq!(queue.pop_last(), Some(5));
        assert_eq!(queue.pop_last(), Some(4));

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_resize_vec_double_queue() {
        let mut queue = VecDoubleQueue::new();

        for i in 0..10 {
            queue.push_first(i);
        }

        for i in (0..10).rev() {
            assert_eq!(queue.pop_first(), Some(i));
        }

        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_vec_double_queue_to_vec() {
        let mut queue = VecDoubleQueue::new();

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);

        queue.push_last(1);
        queue.push_last(2);
        queue.push_last(3);

        assert_eq!(queue.to_vec(), [1, 2, 3]);
    }
}

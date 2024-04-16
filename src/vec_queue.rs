use std::fmt::Debug;

use crate::Queue;

#[derive(Debug)]
pub struct VecQueue<T> {
    data: Vec<T>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T: Copy + Default + Debug> VecQueue<T> {
    pub fn new() -> Self {
        VecQueue {
            data: vec![Default::default(); 3],
            head: 0,
            tail: 0,
            size: 0,
            capacity: 3,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: Copy + Default + Debug> Queue for VecQueue<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        if self.size == self.capacity {
            let new_capacity = self.capacity * 2;
            let mut data = vec![Default::default(); new_capacity];

            for (i, v) in self.to_vec().into_iter().enumerate() {
                data[i] = v;
            }

            self.data = data;
            self.head = 0;
            self.tail = self.size;
            self.capacity = new_capacity;
        }

        self.data[self.tail] = value;

        self.tail = (self.tail + 1) % self.capacity;

        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }

        let value = self.data[self.head];

        self.head = (self.head + 1) % self.capacity;

        self.size -= 1;

        Some(value)
    }

    fn peek(&self) -> Option<Self::Item> {
        self.data.get(self.head).map(|v| *v)
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        let mut result = Vec::with_capacity(self.size);

        let mut i = self.head;
        let mut size = self.size;

        while size > 0 {
            result.push(self.data[i]);

            i = (i + 1) % self.capacity;

            size -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_queue() {
        let mut queue = VecQueue::new();

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);

        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.pop();
        queue.push(3);

        assert_eq!(queue.size(), 3);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.peek(), Some(2));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_resize_vec_queue() {
        let mut queue = VecQueue::new();

        for i in 0..10 {
            queue.push(i);
        }

        for i in 0..10 {
            assert_eq!(queue.pop(), Some(i));
        }

        // assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);
    }
}

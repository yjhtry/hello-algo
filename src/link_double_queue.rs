use std::{cell::RefCell, rc::Rc};

use crate::{DoubleLinkNode, DoubleQueue};

pub struct LinkDoubleQueue<T> {
    front: Option<Rc<RefCell<DoubleLinkNode<T>>>>,
    rear: Option<Rc<RefCell<DoubleLinkNode<T>>>>,
    size: usize,
}

impl<T: Copy> LinkDoubleQueue<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            rear: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: Copy> DoubleQueue for LinkDoubleQueue<T> {
    type Item = T;

    fn push_first(&mut self, value: Self::Item) {
        let node = Rc::new(RefCell::new(DoubleLinkNode {
            value,
            next: self.front.clone(),
            prev: None,
        }));

        if self.size == 0 {
            self.rear = Some(node.clone());
        }

        self.front
            .as_ref()
            .map(|node| node.borrow_mut().prev = Some(node.clone()));

        self.front = Some(node);

        self.size += 1;
    }

    fn push_last(&mut self, value: Self::Item) {
        let node = Rc::new(RefCell::new(DoubleLinkNode {
            value,
            next: None,
            prev: self.rear.clone(),
        }));

        if self.size == 0 {
            self.front = Some(node.clone());
        } else {
            self.rear.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }

        self.rear = Some(node);

        self.size += 1;
    }

    fn peek_first(&self) -> Option<Self::Item> {
        self.front.as_ref().map(|node| node.borrow().value)
    }

    fn peek_last(&self) -> Option<Self::Item> {
        self.rear.as_ref().map(|node| node.borrow().value)
    }

    fn pop_first(&mut self) -> Option<Self::Item> {
        if self.front.is_none() {
            return None;
        }

        let front = self.front.take().unwrap();
        let value = front.borrow().value;

        self.front = front.borrow().next.clone();

        self.size -= 1;

        if self.size == 0 {
            self.rear = None;
        }

        Some(value)
    }

    fn pop_last(&mut self) -> Option<Self::Item> {
        if self.rear.is_none() {
            return None;
        }

        let rear = self.rear.take().unwrap();
        let value = rear.borrow().value;

        self.rear = rear.borrow().prev.clone();

        self.size -= 1;

        if self.size == 0 {
            self.front = None;
        }

        Some(value)
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        let mut result = Vec::with_capacity(self.size);
        let mut front = self.front.clone();

        while let Some(node) = front {
            result.push(node.borrow().value);

            front = node.borrow().next.clone();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_double_queue() {
        let mut queue = LinkDoubleQueue::new();

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
    fn test_resize_link_double_queue() {
        let mut queue = LinkDoubleQueue::new();

        for i in 0..10 {
            queue.push_first(i);
        }

        for i in (0..10).rev() {
            assert_eq!(queue.pop_first(), Some(i));
        }

        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_link_double_queue_to_vec() {
        let mut queue = LinkDoubleQueue::new();

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);

        queue.push_last(1);
        queue.push_last(2);
        queue.push_last(3);

        assert_eq!(queue.to_vec(), [1, 2, 3]);
    }
}

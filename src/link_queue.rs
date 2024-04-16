use std::{cell::RefCell, rc::Rc};

use crate::{LinkNode, Queue};

#[derive(Debug)]
pub struct LinkQueue<T> {
    head: Option<Rc<RefCell<LinkNode<T>>>>,
    tail: Option<Rc<RefCell<LinkNode<T>>>>,
    size: usize,
}

impl<T: Copy> LinkQueue<T> {
    pub fn new() -> Self {
        LinkQueue {
            head: None,
            tail: None,
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

impl<T: Copy + std::fmt::Debug> Queue for LinkQueue<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        let node = Rc::new(RefCell::new(LinkNode { next: None, value }));

        if self.size == 0 {
            self.head = Some(node.clone());
        } else {
            self.tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }

        self.tail = Some(node.clone());

        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }

        let node = self.head.take().unwrap();

        self.head = node.borrow().next.clone();

        if self.size == 1 {
            self.tail = None;
        }

        self.size -= 1;

        return Some(node.borrow().value);
    }

    fn peek(&self) -> Option<Self::Item> {
        self.head.as_ref().map(|node| node.borrow().value)
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        if self.size == 0 {
            return vec![];
        }

        let mut result = Vec::with_capacity(self.size);
        let mut current = self.head.clone();
        while let Some(node) = current {
            result.push(node.borrow().value);
            current = node.borrow().next.clone();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_queue() {
        let mut queue = LinkQueue::new();

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.size(), 3);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.peek(), Some(1));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);

        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);
    }
}

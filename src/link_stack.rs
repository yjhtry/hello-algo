use std::{cell::RefCell, rc::Rc};

use crate::{LinkNode, Stack};

pub struct LinkStack<T> {
    head: Option<Rc<RefCell<LinkNode<T>>>>,
    size: usize,
}

impl<T: Copy> LinkStack<T> {
    pub fn new() -> Self {
        LinkStack {
            head: None,
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

impl<T: Copy> Stack for LinkStack<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        let new_node = Rc::new(RefCell::new(LinkNode {
            next: self.head.take(),
            value,
        }));

        self.head = Some(new_node);

        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.borrow().next.clone();
                self.size -= 1;

                node.borrow_mut().next = None;
                Some(node.borrow().value)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<Self::Item> {
        match self.head.as_ref() {
            Some(node) => Some(node.borrow().value),
            None => None,
        }
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        let mut vec = Vec::with_capacity(self.size);
        let mut current = self.head.clone();

        while let Some(node) = current {
            vec.push(node.borrow().value);
            current = node.borrow().next.clone();
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_stack() {
        let mut stack = LinkStack::new();

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.is_empty(), true);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.to_vec(), vec![3, 2, 1]);
        assert_eq!(stack.peek(), Some(3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.peek(), Some(2));
        assert_eq!(stack.to_vec(), vec![2, 1]);
    }
}

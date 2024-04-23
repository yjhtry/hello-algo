use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

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

        if self.rear.is_none() {
            self.rear = Some(node.clone());
        }

        // self.front
        //     .as_ref()
        //     .map(|node| node.borrow_mut().prev = Some(node.clone()));

        self.front = Some(node);

        self.size += 1;
    }

    fn push_last(&mut self, value: Self::Item) {
        // let node = Rc::new(RefCell::new(DoubleLinkNode { value, next: None }));

        // if self.rear.is_none() {
        //     self.front = Some(node.clone());
        // } else {
        //     self.rear.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        // }
        // self.rear = Some(node);

        // self.size += 1;

        todo!()
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

        Some(value)
    }

    fn pop_last(&mut self) -> Option<Self::Item> {
        // if self.rear.is_none() {
        //     return None;
        // }

        // let rear = self.rear.take().unwrap();
        // let value = rear.borrow().value;

        // self.size -= 1;

        todo!()
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        todo!()
    }
}

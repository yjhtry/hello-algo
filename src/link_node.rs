use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct LinkNode<T> {
    pub next: Option<Rc<RefCell<LinkNode<T>>>>,
    pub value: T,
}

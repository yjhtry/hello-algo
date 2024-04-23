use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct LinkNode<T> {
    pub next: Option<Rc<RefCell<LinkNode<T>>>>,
    pub value: T,
}

#[derive(Debug)]
pub struct DoubleLinkNode<T> {
    pub next: Option<Rc<RefCell<DoubleLinkNode<T>>>>,
    pub prev: Option<Rc<RefCell<DoubleLinkNode<T>>>>,
    pub value: T,
}

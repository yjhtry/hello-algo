use crate::Stack;

pub struct VecStack<T> {
    data: Vec<T>,
}

impl<T: Copy> VecStack<T> {
    pub fn new() -> Self {
        VecStack { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

impl<T: Clone> Stack for VecStack<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }

    fn peek(&self) -> Option<Self::Item> {
        self.data.last().cloned()
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        self.data.clone().into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_stack() {
        let mut stack = VecStack::new();

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.is_empty(), true);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.to_vec(), vec![3, 2, 1]);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.to_vec(), vec![2, 1]);
    }
}

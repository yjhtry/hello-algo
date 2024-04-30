use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TreeNode<T: Copy> {
    pub data: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Copy> TreeNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_level_order() {
        let tree = generator_tree();

        let mut result = vec![];
        let root = tree;
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while let Some(node) = queue.pop_front() {
            result.push(node.borrow().data);

            if let Some(left) = node.borrow().left.as_ref() {
                queue.push_back(left.clone())
            }

            if let Some(right) = node.borrow().right.as_ref() {
                queue.push_back(right.clone())
            }
        }

        assert_eq!(result, [1, 2, 3, 4, 5, 6, 7])
    }

    #[test]
    fn test_depth_order() {
        let tree = generator_tree();
        let res = pre_order(Some(&tree));

        assert_eq!(res, [1, 2, 4, 5, 3, 6, 7]);

        let res = in_order(Some(&tree));

        assert_eq!(res, [4, 2, 5, 1, 6, 3, 7]);

        let res = post_order(Some(&tree));

        assert_eq!(res, [4, 5, 2, 6, 7, 3, 1]);
    }

    fn pre_order<T: Copy>(root: Option<&Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
        let mut result = vec![];

        if let Some(node) = root {
            result.push(node.borrow().data);
            result.extend(pre_order(node.borrow().left.as_ref()));
            result.extend(pre_order(node.borrow().right.as_ref()));
        }

        result
    }
    fn in_order<T: Copy>(root: Option<&Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
        let mut result = vec![];

        if let Some(node) = root {
            result.extend(in_order(node.borrow().left.as_ref()));
            result.push(node.borrow().data);
            result.extend(in_order(node.borrow().right.as_ref()));
        }

        result
    }
    fn post_order<T: Copy>(root: Option<&Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
        let mut result = vec![];

        if let Some(node) = root {
            result.extend(post_order(node.borrow().left.as_ref()));
            result.extend(post_order(node.borrow().right.as_ref()));
            result.push(node.borrow().data);
        }

        result
    }

    fn generator_tree() -> Rc<RefCell<TreeNode<i32>>> {
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));

        node1.borrow_mut().left = Some(node2.clone());
        node1.borrow_mut().right = Some(node3.clone());

        node2.borrow_mut().left = Some(node4.clone());
        node2.borrow_mut().right = Some(node5.clone());

        node3.borrow_mut().left = Some(node6.clone());
        node3.borrow_mut().right = Some(node7.clone());

        node1
    }
}

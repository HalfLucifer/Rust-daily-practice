use std::cell::RefCell;
use std::rc::Rc;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                // Post order travesal
                let l = get_height(&node.borrow().left);
                let r = get_height(&node.borrow().right);
                l.max(r) + 1
            }

            None => 0,
        }
    }

    if let Some(node) = root {
        let left_height = get_height(&node.borrow().left);
        let right_height = get_height(&node.borrow().right);
        return (left_height - right_height).abs() <= 1;
    }

    true
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn attach_left(&mut self, node: TreeNode) {
        if self.left.is_none() {
            self.left = Some(Rc::new(RefCell::new(node)));
        }
    }

    pub fn attach_right(&mut self, node: TreeNode) {
        if self.right.is_none() {
            self.right = Some(Rc::new(RefCell::new(node)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut root = TreeNode::new(0);
        let mut left1 = TreeNode::new(1);
        let mut right1 = TreeNode::new(2);
        let left2 = TreeNode::new(3);
        let left3 = TreeNode::new(4);
        let right2 = TreeNode::new(5);
        let right3 = TreeNode::new(6);

        left1.attach_left(left2);
        left1.attach_right(left3);
        right1.attach_left(right2);
        right1.attach_right(right3);
        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(root)))), true);
    }

    #[test]
    fn test_diff_one_cases() {
        let mut root = TreeNode::new(0);
        let mut left1 = TreeNode::new(1);
        let mut right1 = TreeNode::new(2);
        let left2 = TreeNode::new(3);
        let left3 = TreeNode::new(4);
        let right2 = TreeNode::new(5);

        left1.attach_left(left2);
        left1.attach_right(left3);
        right1.attach_left(right2);
        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(root)))), true);
    }

    #[test]
    fn test_failed_cases() {
        let mut root = TreeNode::new(0);
        let mut left1 = TreeNode::new(1);
        let mut left2 = TreeNode::new(2);
        let mut left3 = TreeNode::new(3);
        let left4 = TreeNode::new(4);
        let right1 = TreeNode::new(5);

        left3.attach_left(left4);
        left1.attach_left(left2);
        left1.attach_right(left3);
        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(root)))), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_balanced(None), true);

        let root = TreeNode::new(0);
        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(root)))), true);

        let mut root = TreeNode::new(0);
        let child = TreeNode::new(1);
        root.attach_right(child);
        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(root)))), true);

        let mut root = TreeNode::new(0);
        let mut child = TreeNode::new(1);
        let grandchild = TreeNode::new(2);
        child.attach_right(grandchild);
        root.attach_right(child);
        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(root)))), false);
    }
}

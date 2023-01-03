use std::cell::RefCell;
use std::rc::Rc;

pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_identical(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub {
            return true;
        }

        match (root, sub) {
            (Some(node), Some(_)) => {
                is_identical(&node.borrow().left, sub) || is_identical(&node.borrow().right, sub)
            }

            (None, None) => true,

            _ => false,
        }
    }

    is_identical(&root, &sub)
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

        let mut sub = TreeNode::new(1);
        let sub_l = TreeNode::new(3);
        let sub_r = TreeNode::new(4);
        sub.attach_left(sub_l);
        sub.attach_right(sub_r);

        assert_eq!(
            is_subtree(
                Some(Rc::new(RefCell::new(root))),
                Some(Rc::new(RefCell::new(sub)))
            ),
            true
        );
    }

    #[test]
    fn test_identical_cases() {
        let mut root1 = TreeNode::new(0);
        let mut left1 = TreeNode::new(1);
        let mut right1 = TreeNode::new(2);
        let right2 = TreeNode::new(5);
        let right3 = TreeNode::new(6);
        right1.attach_left(right2);
        right1.attach_right(right3);
        root1.attach_left(left1);
        root1.attach_right(right1);

        let mut root2 = TreeNode::new(0);
        let mut left1 = TreeNode::new(1);
        let mut right1 = TreeNode::new(2);
        let right2 = TreeNode::new(5);
        let right3 = TreeNode::new(6);
        right1.attach_left(right2);
        right1.attach_right(right3);
        root2.attach_left(left1);
        root2.attach_right(right1);

        assert_eq!(
            is_subtree(
                Some(Rc::new(RefCell::new(root1))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );
    }

    #[test]
    fn test_edge_cases() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let sub = Some(Rc::new(RefCell::new(TreeNode::new(0))));

        assert_eq!(is_subtree(None, None), true);
        assert_eq!(is_subtree(root.clone(), None), false);
        assert_eq!(is_subtree(None, sub.clone()), false);
        assert_eq!(is_subtree(root, sub), true);
    }
}

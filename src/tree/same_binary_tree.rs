use std::cell::RefCell;
use std::rc::Rc;

pub fn is_same_tree(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (root1, root2) {
        (Some(p), Some(q)) => {
            if p.borrow().val != q.borrow().val {
                return false;
            }

            let l = is_same_tree(p.borrow_mut().left.take(), q.borrow_mut().left.take());
            let r = is_same_tree(p.borrow_mut().right.take(), q.borrow_mut().right.take());

            l && r
        }

        (None, None) => true,

        _ => false,
    }
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
        let mut root1 = TreeNode::new(0);
        let mut root2 = TreeNode::new(0);

        let mut left1 = TreeNode::new(1);
        let left2 = TreeNode::new(2);
        let mut left3 = TreeNode::new(1);
        let left4 = TreeNode::new(2);

        left1.attach_left(left2);
        root1.attach_left(left1);

        left3.attach_left(left4);
        root2.attach_left(left3);

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(root1))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );
    }

    #[test]
    fn test_larger_cases() {
        let mut root1 = TreeNode::new(0);
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
        root1.attach_left(left1);
        root1.attach_right(right1);

        let mut root2 = TreeNode::new(0);
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
        root2.attach_left(left1);
        root2.attach_right(right1);

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(root1))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_same_tree(None, None), true);

        assert_eq!(
            is_same_tree(Some(Rc::new(RefCell::new(TreeNode::new(0)))), None),
            false
        );
    }
}

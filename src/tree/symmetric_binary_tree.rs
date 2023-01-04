use std::cell::RefCell;
use std::rc::Rc;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn compare(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(l), Some(r)) => {
                if l.borrow().val != r.borrow().val {
                    false
                } else {
                    let ll = l.borrow_mut().left.take();
                    let lr = l.borrow_mut().right.take();
                    let rl = r.borrow_mut().left.take();
                    let rr = r.borrow_mut().right.take();
    
                    compare(ll, rr) && compare(lr, rl)    
                }
            }

            (None, None) => true,

            _ => false,
        }
    }

    if let Some(node) = root {
        let l = node.borrow_mut().left.take();
        let r = node.borrow_mut().right.take();
        compare(l, r)
    } else {
        false
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
        let mut root = TreeNode::new(0);
        let left1 = TreeNode::new(1);
        let right1 = TreeNode::new(1);
        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(root)))), true);

        let mut root = TreeNode::new(0);
        let left1 = TreeNode::new(1);
        let right1 = TreeNode::new(2);
        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(root)))), false);
    }

    #[test]
    fn test_larger_cases() {
        let mut root = TreeNode::new(0);
        let mut left1 = TreeNode::new(1);
        let mut right1 = TreeNode::new(1);

        let left2 = TreeNode::new(2);
        let left3 = TreeNode::new(3);
        let right2 = TreeNode::new(3);
        let right3 = TreeNode::new(2);

        left1.attach_left(left2);
        left1.attach_right(left3);
        right1.attach_left(right2);
        right1.attach_right(right3);
        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(root)))), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_symmetric(None), false);

        let root = TreeNode::new(0);
        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(root)))), true);
    }
}

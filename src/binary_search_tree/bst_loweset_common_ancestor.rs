use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root == p || root == q {
        return root;
    }

    let node = root.as_ref().unwrap().borrow();
    let r_val = node.val;
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    if r_val > p_val && r_val > q_val {
        lowest_common_ancestor(node.left.clone(), p, q)
    } else if r_val < p_val && r_val < q_val {
        lowest_common_ancestor(node.right.clone(), p, q)
    } else {
        root.clone()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        let left1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let right1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let left2 = Rc::new(RefCell::new(TreeNode::new(0)));
        let right2 = Rc::new(RefCell::new(TreeNode::new(4)));

        root.borrow_mut().left = Some(Rc::clone(&left1));
        root.borrow_mut().right = Some(Rc::clone(&right1));
        left1.borrow_mut().left = Some(Rc::clone(&left2));
        right1.borrow_mut().right = Some(Rc::clone(&right2));

        let node = lowest_common_ancestor(
            Some(root.clone()),
            Some(left1.clone()),
            Some(right1.clone()),
        );
        assert_eq!(node.unwrap().borrow().val, 2);

        let node =
            lowest_common_ancestor(Some(root.clone()), Some(root.clone()), Some(right1.clone()));
        assert_eq!(node.unwrap().borrow().val, 2);

        let node =
            lowest_common_ancestor(Some(root.clone()), Some(left1.clone()), Some(root.clone()));
        assert_eq!(node.unwrap().borrow().val, 2);

        let node = lowest_common_ancestor(
            Some(root.clone()),
            Some(left2.clone()),
            Some(right2.clone()),
        );
        assert_eq!(node.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_edge_cases() {
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        let node =
            lowest_common_ancestor(Some(root.clone()), Some(root.clone()), Some(root.clone()));
        assert_eq!(node.unwrap().borrow().val, 0);
    }
}

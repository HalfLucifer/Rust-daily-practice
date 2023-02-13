use std::cell::RefCell;
use std::rc::Rc;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn bst_recursively(nums: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }

        let mid = (left + right) / 2;
        let mut root = TreeNode::new(nums[mid as usize]);

        root.left = bst_recursively(nums, left, mid - 1);
        root.right = bst_recursively(nums, mid + 1, right);

        Some(Rc::new(RefCell::new(root)))
    }

    let len = nums.len() as i32;
    bst_recursively(&nums, 0, len - 1)
}

pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traverse_inorder(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(node) = root {
            if let Some(l) = node.borrow().left.clone() {
                traverse_inorder(Some(l), arr);
            }

            arr.push(node.borrow().val);

            if let Some(r) = node.borrow().right.clone() {
                traverse_inorder(Some(r), arr);
            }
        }
    }

    let mut ret = vec![];
    traverse_inorder(root, &mut ret);
    ret
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
        let input = vec![-10, -3, 0, 5, 9];
        let root = sorted_array_to_bst(input.clone());
        assert_eq!(traverse(root), input);

        let input = vec![1, 3];
        let root = sorted_array_to_bst(input.clone());
        assert_eq!(traverse(root), input);

        let input = (1..100).collect::<Vec<_>>();
        let root = sorted_array_to_bst(input.clone());
        assert_eq!(traverse(root), input);
    }

    #[test]
    fn test_edge_cases() {
        let root = sorted_array_to_bst(vec![0]);
        assert_eq!(traverse(root), vec![0]);

        let root = sorted_array_to_bst(vec![i32::MIN, i32::MAX]);
        assert_eq!(traverse(root), vec![i32::MIN, i32::MAX]);
    }
}

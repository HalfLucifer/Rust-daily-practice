pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Copy + std::cmp::PartialEq> TreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            left: None,
            right: None,
        }
    }

    pub fn attach_left(&mut self, node: TreeNode<T>) {
        if self.left.is_none() {
            self.left = Some(Box::new(node));
        }
    }

    pub fn attach_right(&mut self, node: TreeNode<T>) {
        if self.right.is_none() {
            self.right = Some(Box::new(node));
        }
    }

    /*
       Assumption: No TreeNode has the same value
    */
    pub fn find_lowest_common_ancestor(&self, value_a: T, value_b: T) -> Option<T> {
        fn find_lowest_common_ancestor_recursive<'a, T: std::cmp::PartialEq>(
            root: &'a TreeNode<T>,
            a: &'a TreeNode<T>,
            b: &'a TreeNode<T>,
        ) -> Option<&'a TreeNode<T>> {
            if root.value == a.value || root.value == b.value {
                return Some(root);
            }

            let left = {
                if let Some(node) = &root.left {
                    find_lowest_common_ancestor_recursive(node, a, b)
                } else {
                    None
                }
            };

            let right = {
                if let Some(node) = &root.right {
                    find_lowest_common_ancestor_recursive(node, a, b)
                } else {
                    None
                }
            };

            match (left, right) {
                (Some(_), Some(_)) => Some(root),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }

        match find_lowest_common_ancestor_recursive(
            self,
            &TreeNode::new(value_a),
            &TreeNode::new(value_b),
        ) {
            Some(node) => Some(node.value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut root: TreeNode<i32> = TreeNode::new(0);
        let mut left1: TreeNode<i32> = TreeNode::new(1);
        let mut right1: TreeNode<i32> = TreeNode::new(2);
        let left2: TreeNode<i32> = TreeNode::new(3);
        let left3: TreeNode<i32> = TreeNode::new(4);
        let right2: TreeNode<i32> = TreeNode::new(5);
        let right3: TreeNode<i32> = TreeNode::new(6);

        left1.attach_left(left2);
        left1.attach_right(left3);
        right1.attach_left(right2);
        right1.attach_right(right3);
        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(root.find_lowest_common_ancestor(1, 2), Some(0));
        assert_eq!(root.find_lowest_common_ancestor(1, 6), Some(0));
        assert_eq!(root.find_lowest_common_ancestor(1, 4), Some(1));
        assert_eq!(root.find_lowest_common_ancestor(3, 4), Some(1));
        assert_eq!(root.find_lowest_common_ancestor(2, 5), Some(2));
        assert_eq!(root.find_lowest_common_ancestor(5, 6), Some(2));

        assert_eq!(root.find_lowest_common_ancestor(-100, 2), Some(2));
        assert_eq!(root.find_lowest_common_ancestor(3, 100), Some(3));
        assert_eq!(root.find_lowest_common_ancestor(-100, 100), None);
    }

    #[test]
    fn test_edge_cases() {
        let root: TreeNode<i32> = TreeNode::new(0);

        assert_eq!(root.find_lowest_common_ancestor(0, 0), Some(0));
        assert_eq!(root.find_lowest_common_ancestor(-1, 0), Some(0));
        assert_eq!(root.find_lowest_common_ancestor(0, 1), Some(0));
        assert_eq!(root.find_lowest_common_ancestor(0, i32::MAX), Some(0));
        assert_eq!(root.find_lowest_common_ancestor(0, i32::MIN), Some(0));
        assert_eq!(root.find_lowest_common_ancestor(i32::MIN, i32::MAX), None);
    }
}

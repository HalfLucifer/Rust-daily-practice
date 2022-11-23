pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Copy> TreeNode<T> {
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

    pub fn depth_first_traverse(&self) -> Vec<T> {
        fn depth_first_traverse_recursive<T: Copy>(
            root: &TreeNode<T>,
            level: usize,
            result: &mut Vec<T>,
        ) {
            if result.len() == level {
                result.push(root.value);
            }

            if let Some(node) = &root.right {
                depth_first_traverse_recursive(node, level + 1, result);
            }

            if let Some(node) = &root.left {
                depth_first_traverse_recursive(node, level + 1, result);
            }
        }

        let mut result = vec![];
        depth_first_traverse_recursive(&self, 0, &mut result);
        result
    }

    pub fn breadth_first_traverse(&self) -> Vec<T> {
        let mut result = vec![];
        let mut queue = vec![self];

        while !queue.is_empty() {
            let mut current_count = 0;
            let total_count = queue.len();

            while current_count < total_count {
                let root = queue.remove(0);

                if current_count == 0 {
                    result.push(root.value);
                }

                if let Some(node) = &root.right {
                    queue.push(node);
                }

                if let Some(node) = &root.left {
                    queue.push(node);
                }

                current_count += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        /*
                  0
                 / \
                1   2
               / \ / \
              3  4 5  6
             /      \
            7        8

            right view: 0,2,6,8
        */
        let mut root: TreeNode<i32> = TreeNode::new(0);
        let mut left1: TreeNode<i32> = TreeNode::new(1);
        let mut right1: TreeNode<i32> = TreeNode::new(2);
        let mut left2: TreeNode<i32> = TreeNode::new(3);
        let left3: TreeNode<i32> = TreeNode::new(4);
        let mut right2: TreeNode<i32> = TreeNode::new(5);
        let right3: TreeNode<i32> = TreeNode::new(6);
        let left4: TreeNode<i32> = TreeNode::new(7);
        let right4: TreeNode<i32> = TreeNode::new(8);

        left2.attach_left(left4);
        right2.attach_right(right4);

        left1.attach_left(left2);
        left1.attach_right(left3);

        right1.attach_left(right2);
        right1.attach_right(right3);

        root.attach_left(left1);
        root.attach_right(right1);

        assert_eq!(root.depth_first_traverse(), [0, 2, 6, 8]);
        assert_eq!(root.breadth_first_traverse(), [0, 2, 6, 8]);
    }

    #[test]
    fn test_edge_cases() {
        let mut root: TreeNode<i32> = TreeNode::new(0);
        assert_eq!(root.depth_first_traverse(), [0]);
        assert_eq!(root.breadth_first_traverse(), [0]);

        let left1: TreeNode<i32> = TreeNode::new(1);
        root.attach_left(left1);
        assert_eq!(root.depth_first_traverse(), [0, 1]);
        assert_eq!(root.breadth_first_traverse(), [0, 1]);

        let right1: TreeNode<i32> = TreeNode::new(2);
        root.attach_right(right1);
        assert_eq!(root.depth_first_traverse(), [0, 2]);
        assert_eq!(root.breadth_first_traverse(), [0, 2]);
    }

    #[test]
    fn test_single_child() {
        let mut root: TreeNode<i32> = TreeNode::new(0);
        let mut node1: TreeNode<i32> = TreeNode::new(10);
        let mut node2: TreeNode<i32> = TreeNode::new(20);
        let mut node3: TreeNode<i32> = TreeNode::new(30);
        let node4: TreeNode<i32> = TreeNode::new(40);

        node3.attach_right(node4);
        node2.attach_left(node3);
        node1.attach_right(node2);
        root.attach_left(node1);

        assert_eq!(root.depth_first_traverse(), [0, 10, 20, 30, 40]);
        assert_eq!(root.breadth_first_traverse(), [0, 10, 20, 30, 40]);
    }
}

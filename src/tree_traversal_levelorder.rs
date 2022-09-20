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

    #[allow(dead_code)]
    pub fn detach_left(&mut self) {
        self.left = None;
    }

    #[allow(dead_code)]
    pub fn detach_right(&mut self) {
        self.right = None;
    }

    pub fn traverse_levelorder(&self) -> Vec<T> {
        let mut ret = vec![];
        let mut queue = vec![self];

        while !queue.is_empty() {
            let root = queue.remove(0);
            ret.push(root.value);

            if let Some(node) = &root.left {
                queue.push(node);
            }

            if let Some(node) = &root.right {
                queue.push(node);
            }
        }

        ret
    }

    pub fn traverse_levelorder_subarray(&self) -> Vec<Vec<T>> {
        let mut ret = vec![];
        let mut queue = vec![self];

        while !queue.is_empty() {
            let mut subarray = vec![];
            let mut current_count = 0;
            let total_count = queue.len();

            while current_count < total_count {
                let root = queue.remove(0);
                subarray.push(root.value);

                if let Some(node) = &root.left {
                    queue.push(node);
                }

                if let Some(node) = &root.right {
                    queue.push(node);
                }

                current_count += 1;
            }

            ret.push(subarray);
        }

        ret
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

            level order traversal: 0,1,2,3,4,5,6
            level order subarray: [[0], [1,2], [3,4,5,6]]
        */
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

        assert_eq!(root.traverse_levelorder(), [0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(
            root.traverse_levelorder_subarray(),
            [vec![0], vec![1, 2], vec![3, 4, 5, 6]]
        );
    }

    #[test]
    fn test_edge_cases() {
        let mut root: TreeNode<i32> = TreeNode::new(0);
        assert_eq!(root.traverse_levelorder(), [0]);
        assert_eq!(root.traverse_levelorder_subarray(), [vec![0]]);

        let left1: TreeNode<i32> = TreeNode::new(1);
        root.attach_left(left1);
        assert_eq!(root.traverse_levelorder(), [0, 1]);
        assert_eq!(root.traverse_levelorder_subarray(), [vec![0], vec![1]]);

        let right1: TreeNode<i32> = TreeNode::new(2);
        root.attach_right(right1);
        assert_eq!(root.traverse_levelorder(), [0, 1, 2]);
        assert_eq!(root.traverse_levelorder_subarray(), [vec![0], vec![1, 2]]);
    }

    #[test]
    fn test_left_children() {
        let mut root: TreeNode<i32> = TreeNode::new(0);
        let mut left1: TreeNode<i32> = TreeNode::new(10);
        let mut left2: TreeNode<i32> = TreeNode::new(20);
        let mut left3: TreeNode<i32> = TreeNode::new(30);
        let left4: TreeNode<i32> = TreeNode::new(40);

        left3.attach_left(left4);
        left2.attach_left(left3);
        left1.attach_left(left2);
        root.attach_left(left1);

        assert_eq!(root.traverse_levelorder(), [0, 10, 20, 30, 40]);
        assert_eq!(
            root.traverse_levelorder_subarray(),
            [vec![0], vec![10], vec![20], vec![30], vec![40]]
        );
    }
}

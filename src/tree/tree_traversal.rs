pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Clone> TreeNode<T> {
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

    pub fn traverse_preorder(&self) -> Vec<T> {
        fn traverse_preorder_recursive<T: Clone>(root: &TreeNode<T>, arr: &mut Vec<T>) {
            arr.push(root.value.clone());

            if let Some(node) = &root.left {
                traverse_preorder_recursive(node, arr);
            }

            if let Some(node) = &root.right {
                traverse_preorder_recursive(node, arr);
            }
        }

        let mut ret = vec![];
        traverse_preorder_recursive(&self, &mut ret);
        ret
    }

    pub fn traverse_inorder(&self) -> Vec<T> {
        fn traverse_inorder_recursive<T: Clone>(root: &TreeNode<T>, arr: &mut Vec<T>) {
            if let Some(node) = &root.left {
                traverse_inorder_recursive(node, arr);
            }

            arr.push(root.value.clone());

            if let Some(node) = &root.right {
                traverse_inorder_recursive(node, arr);
            }
        }

        let mut ret = vec![];
        traverse_inorder_recursive(&self, &mut ret);
        ret
    }

    pub fn traverse_postorder(&self) -> Vec<T> {
        fn traverse_postorder_recursive<T: Clone>(root: &TreeNode<T>, arr: &mut Vec<T>) {
            if let Some(node) = &root.left {
                traverse_postorder_recursive(node, arr);
            }

            if let Some(node) = &root.right {
                traverse_postorder_recursive(node, arr);
            }

            arr.push(root.value.clone());
        }

        let mut ret = vec![];
        traverse_postorder_recursive(&self, &mut ret);
        ret
    }

    pub fn get_height(&self) -> usize {
        fn get_height_recursive<T>(root: &Option<Box<TreeNode<T>>>) -> usize {
            match root {
                Some(node) => {
                    let left_height = get_height_recursive(&node.left);
                    let right_height = get_height_recursive(&node.right);
                    1 + left_height.max(right_height)
                }
                None => 0,
            }
        }

        let left_height = get_height_recursive(&self.left);
        let right_height = get_height_recursive(&self.right);
        1 + left_height.max(right_height)
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

            preorder traversal:    0,1,3,4,2,5,6
            inorder traversal:     3,1,4,0,5,2,6
            postorder traversal:   3,4,1,5,6,2,0
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

        assert_eq!(root.traverse_preorder(), [0, 1, 3, 4, 2, 5, 6]);
        assert_eq!(root.traverse_inorder(), [3, 1, 4, 0, 5, 2, 6]);
        assert_eq!(root.traverse_postorder(), [3, 4, 1, 5, 6, 2, 0]);
        assert_eq!(root.get_height(), 3);
    }

    #[test]
    fn test_edge_cases() {
        let mut root: TreeNode<i32> = TreeNode::new(0);

        assert_eq!(root.traverse_preorder(), [0]);
        assert_eq!(root.traverse_inorder(), [0]);
        assert_eq!(root.traverse_postorder(), [0]);
        assert_eq!(root.get_height(), 1);

        let left1: TreeNode<i32> = TreeNode::new(1);
        root.attach_left(left1);

        assert_eq!(root.traverse_preorder(), [0, 1]);
        assert_eq!(root.traverse_inorder(), [1, 0]);
        assert_eq!(root.traverse_postorder(), [1, 0]);
        assert_eq!(root.get_height(), 2);

        let right1: TreeNode<i32> = TreeNode::new(2);
        root.attach_right(right1);

        assert_eq!(root.traverse_preorder(), [0, 1, 2]);
        assert_eq!(root.traverse_inorder(), [1, 0, 2]);
        assert_eq!(root.traverse_postorder(), [1, 2, 0]);
        assert_eq!(root.get_height(), 2);
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

        assert_eq!(root.traverse_preorder(), [0, 10, 20, 30, 40]);
        assert_eq!(root.traverse_inorder(), [40, 30, 20, 10, 0]);
        assert_eq!(root.traverse_postorder(), [40, 30, 20, 10, 0]);
        assert_eq!(root.get_height(), 5);
    }
}

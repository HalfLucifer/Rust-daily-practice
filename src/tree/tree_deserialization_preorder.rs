pub struct TreeNode {
    value: String,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    const EMPTY_NODE: &'static str = "#";
    const SEP: &'static str = ",";

    pub fn new(init: &str) -> Self {
        Self {
            value: init.to_owned(),
            left: None,
            right: None,
        }
    }

    pub fn attach_left(&mut self, node: TreeNode) {
        if self.left.is_none() {
            self.left = Some(Box::new(node));
        }
    }

    pub fn attach_right(&mut self, node: TreeNode) {
        if self.right.is_none() {
            self.right = Some(Box::new(node));
        }
    }

    pub fn serialize_preorder(root: TreeNode) -> String {
        fn serialize_preorder_recursive(root: Option<Box<TreeNode>>, ret: &mut Vec<String>) {
            match root {
                Some(node) => {
                    ret.push(node.value);
                    serialize_preorder_recursive(node.left, ret);
                    serialize_preorder_recursive(node.right, ret);
                }
                None => {
                    ret.push(TreeNode::EMPTY_NODE.to_string());
                }
            }
        }

        let mut ret = vec![];
        serialize_preorder_recursive(Some(Box::new(root)), &mut ret);
        ret.join(TreeNode::SEP)
    }

    pub fn deserialize_preorder(input: &str) -> Option<Box<TreeNode>> {
        fn deserialize_preorder_recursive(arr: &mut Vec<String>) -> Option<Box<TreeNode>> {
            if arr.is_empty() {
                return None;
            }

            let s = arr.remove(0);
            if s == TreeNode::EMPTY_NODE {
                return None;
            }

            Some(Box::new(TreeNode {
                value: s,
                left: deserialize_preorder_recursive(arr),
                right: deserialize_preorder_recursive(arr),
            }))
        }

        let mut arr = input.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
        deserialize_preorder_recursive(&mut arr)
    }

    pub fn traverse_preorder(&self) -> Vec<String> {
        let mut ret = vec![];
        self.traverse_preorder_recursive(&mut ret);
        ret
    }

    fn traverse_preorder_recursive(&self, arr: &mut Vec<String>) {
        arr.push(self.value.clone());

        if let Some(node) = &self.left {
            node.traverse_preorder_recursive(arr);
        }

        if let Some(node) = &self.right {
            node.traverse_preorder_recursive(arr);
        }
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

            preorder traversal: 0,1,3,4,2,5,6
        */
        let mut root = TreeNode::new("0");
        let mut left1 = TreeNode::new("1");
        let mut right1 = TreeNode::new("2");
        let left2 = TreeNode::new("3");
        let left3 = TreeNode::new("4");
        let right2 = TreeNode::new("5");
        let right3 = TreeNode::new("6");

        left1.attach_left(left2);
        left1.attach_right(left3);
        right1.attach_left(right2);
        right1.attach_right(right3);
        root.attach_left(left1);
        root.attach_right(right1);

        let input = "0,1,3,#,#,4,#,#,2,5,#,#,6,#,#";
        let expected = root.traverse_preorder();
        let serialized = TreeNode::serialize_preorder(root);
        let deserialized = TreeNode::deserialize_preorder(input);

        assert_eq!(serialized, input);
        assert_eq!(deserialized.unwrap().traverse_preorder(), expected);
    }

    #[test]
    fn test_abc_cases() {
        /*
                a
               / \
              b   e
             / \ / \
            c  d f  g

            preorder traversal: a,b,c,d,e,f,g
        */
        let mut root = TreeNode::new("a");
        let mut left1 = TreeNode::new("b");
        let mut right1 = TreeNode::new("e");
        let left2 = TreeNode::new("c");
        let left3 = TreeNode::new("d");
        let right2 = TreeNode::new("f");
        let right3 = TreeNode::new("g");

        left1.attach_left(left2);
        left1.attach_right(left3);
        right1.attach_left(right2);
        right1.attach_right(right3);
        root.attach_left(left1);
        root.attach_right(right1);

        let input = "a,b,c,#,#,d,#,#,e,f,#,#,g,#,#";
        let expected = root.traverse_preorder();
        let serialized = TreeNode::serialize_preorder(root);
        let deserialized = TreeNode::deserialize_preorder(input);

        assert_eq!(serialized, input);
        assert_eq!(deserialized.unwrap().traverse_preorder(), expected);
    }

    #[test]
    fn test_right_children() {
        let mut root = TreeNode::new("I");
        let mut c1 = TreeNode::new("love");
        let mut c2 = TreeNode::new("coding");
        let mut c3 = TreeNode::new("in");
        let c4 = TreeNode::new("Rust!");

        c3.attach_right(c4);
        c2.attach_right(c3);
        c1.attach_right(c2);
        root.attach_right(c1);

        let input = "I,#,love,#,coding,#,in,#,Rust!,#,#";
        let expected = root.traverse_preorder();
        let serialized = TreeNode::serialize_preorder(root);
        let deserialized = TreeNode::deserialize_preorder(input);

        assert_eq!(serialized, input);
        assert_eq!(deserialized.unwrap().traverse_preorder(), expected);
    }

    #[test]
    fn test_edge_cases() {
        let root = TreeNode::new("Rust");
        let expected = root.traverse_preorder();
        let serialized = TreeNode::serialize_preorder(root);
        let deserialized = TreeNode::deserialize_preorder("Rust");

        assert_eq!(serialized, "Rust,#,#");
        assert_eq!(deserialized.unwrap().traverse_preorder(), expected);
    }
}

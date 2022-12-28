pub struct BinarySearchTree {
    root: BSTNode,
}

impl BinarySearchTree {
    pub fn new(value: i32) -> Self {
        Self {
            root: BSTNode {
                value: value,
                left: None,
                right: None,
            },
        }
    }

    pub fn insert(&mut self, value: i32) {
        fn insert_recursive(root: &mut BSTNode, value: i32) -> &mut BSTNode {
            match root.value.cmp(&value) {
                std::cmp::Ordering::Greater => {
                    if root.left.is_some() {
                        return insert_recursive(root.left.as_mut().unwrap(), value);
                    }
                }

                std::cmp::Ordering::Less => {
                    if root.right.is_some() {
                        return insert_recursive(root.right.as_mut().unwrap(), value);
                    }
                }

                std::cmp::Ordering::Equal => {}
            }

            root
        }

        let mut node = insert_recursive(&mut self.root, value);

        match node.value.cmp(&value) {
            std::cmp::Ordering::Greater => {
                node.left = Some(Box::new(BSTNode {
                    value: value,
                    left: None,
                    right: None,
                }));
            }

            std::cmp::Ordering::Less => {
                node.right = Some(Box::new(BSTNode {
                    value: value,
                    left: None,
                    right: None,
                }));
            }

            std::cmp::Ordering::Equal => {}
        }
    }

    pub fn search(&self, value: i32) -> bool {
        fn search_recursive(root: &BSTNode, value: i32) -> bool {
            match root.value.cmp(&value) {
                std::cmp::Ordering::Greater => {
                    if let Some(node) = &root.left {
                        search_recursive(node, value)
                    } else {
                        false
                    }
                }

                std::cmp::Ordering::Less => {
                    if let Some(node) = &root.right {
                        search_recursive(node, value)
                    } else {
                        false
                    }
                }

                std::cmp::Ordering::Equal => true,
            }
        }

        search_recursive(&self.root, value)
    }

    pub fn validate(&self) -> bool {
        fn validate_recursive(
            root: Option<&BSTNode>,
            min: Option<&BSTNode>,
            max: Option<&BSTNode>,
        ) -> bool {
            let mut result = true;

            match root {
                Some(node) => {
                    if let Some(min_node) = min {
                        if node.value < min_node.value {
                            return false;
                        }
                    }

                    if let Some(max_node) = max {
                        if node.value > max_node.value {
                            return false;
                        }
                    }

                    if let Some(left_node) = &node.left {
                        result = validate_recursive(Some(left_node), min, Some(node));
                    }

                    if let Some(right_node) = &node.right {
                        result = result & validate_recursive(Some(right_node), Some(node), max);
                    }
                }

                None => { /* return true; */ }
            }

            result
        }

        validate_recursive(Some(&self.root), None, None)
    }

    // In-order travese equals to ascending order traverse
    pub fn traverse(&self) -> Vec<i32> {
        fn traverse_inorder_recursive(root: &BSTNode, arr: &mut Vec<i32>) {
            if let Some(node) = &root.left {
                traverse_inorder_recursive(node, arr);
            }

            arr.push(root.value);

            if let Some(node) = &root.right {
                traverse_inorder_recursive(node, arr);
            }
        }

        let mut ret = vec![];
        traverse_inorder_recursive(&self.root, &mut ret);
        ret
    }
}

struct BSTNode {
    value: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut bst = BinarySearchTree::new(0);
        let mut input = [0, 1, 3, -5, 7, -9, 11, -13, 15, -17, 19];
        for i in input.iter() {
            bst.insert(*i);
        }

        assert!(bst.validate());

        for i in -20..20 {
            assert_eq!(bst.search(i), input.contains(&i));
        }

        input.sort();
        assert_eq!(bst.traverse(), input);
    }

    #[test]
    fn test_less_than_root_cases() {
        let mut bst = BinarySearchTree::new(100);
        for i in (1..=100).rev() {
            bst.insert(i);
        }

        for i in 1..=100 {
            assert!(bst.search(i));
        }

        assert!(bst.validate());
        assert_eq!(bst.traverse(), (1..=100).collect::<Vec<_>>());
    }

    #[test]
    fn test_greater_than_root_cases() {
        let mut bst = BinarySearchTree::new(-100);
        for i in (-100..=0).rev() {
            bst.insert(i);
        }

        for i in -100..=0 {
            assert!(bst.search(i));
        }

        assert!(bst.validate());
        assert_eq!(bst.traverse(), (-100..=0).collect::<Vec<_>>());
    }

    #[test]
    fn test_edge_cases() {
        let mut bst = BinarySearchTree::new(0);
        bst.insert(0);
        assert!(bst.search(0));
        assert!(bst.validate());
        assert_eq!(bst.traverse(), [0]);
    }
}

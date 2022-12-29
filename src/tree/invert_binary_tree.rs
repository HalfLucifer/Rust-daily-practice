use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    value: String,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    const EMPTY_NODE: &'static str = "#";

    pub fn new(init: &str) -> Self {
        Self {
            value: init.to_owned(),
            left: None,
            right: None,
        }
    }

    pub fn attach_left(&mut self, node: Rc<RefCell<TreeNode>>) {
        self.left = Some(node);
    }

    pub fn attach_right(&mut self, node: Rc<RefCell<TreeNode>>) {
        self.right = Some(node);
    }

    pub fn invert(node: Rc<RefCell<TreeNode>>) {
        fn invert_recursive(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = node {
                let mut root = n.borrow_mut();

                invert_recursive(&mut root.left);
                invert_recursive(&mut root.right);

                match (root.left.take(), root.right.take()) {
                    (None, None) => {}

                    (l, r) => {
                        root.left = r;
                        root.right = l;
                    }
                }
            }
        }

        invert_recursive(&mut Some(node));
    }

    pub fn traverse_level_order(root: Rc<RefCell<TreeNode>>) -> Vec<String> {
        let mut ret = vec![];
        let mut queue = vec![];
        let empty_node = Rc::new(RefCell::new(TreeNode::new(TreeNode::EMPTY_NODE)));

        queue.push(root);

        while !queue.is_empty() {
            let parent = queue.remove(0);
            let root_node = parent.borrow();

            ret.push(root_node.value.clone());

            if root_node.value == TreeNode::EMPTY_NODE {
                continue;
            }

            if let Some(node) = &root_node.left {
                queue.push(Rc::clone(node));
            } else {
                queue.push(Rc::clone(&empty_node));
            }

            if let Some(node) = &root_node.right {
                queue.push(Rc::clone(node));
            } else {
                queue.push(Rc::clone(&empty_node));
            }
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
                0              0
               / \            / \
              1   2    =>    2   1
             / \ / \        / \ / \
            3  4 5  6      6  5 4  3

            level order traversal: 0,1,2,3,4,5,6 => 0,2,1,6,5,4,3
        */
        let root = Rc::new(RefCell::new(TreeNode::new("0")));
        let left1 = Rc::new(RefCell::new(TreeNode::new("1")));
        let right1 = Rc::new(RefCell::new(TreeNode::new("2")));
        let left2 = Rc::new(RefCell::new(TreeNode::new("3")));
        let left3 = Rc::new(RefCell::new(TreeNode::new("4")));
        let right2 = Rc::new(RefCell::new(TreeNode::new("5")));
        let right3 = Rc::new(RefCell::new(TreeNode::new("6")));

        root.borrow_mut().attach_left(left1.clone());
        root.borrow_mut().attach_right(right1.clone());
        left1.borrow_mut().attach_left(left2.clone());
        left1.borrow_mut().attach_right(left3.clone());
        right1.borrow_mut().attach_left(right2.clone());
        right1.borrow_mut().attach_right(right3.clone());

        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["0", "1", "2", "3", "4", "5", "6", "#", "#", "#", "#", "#", "#", "#", "#"]
        );

        TreeNode::invert(Rc::clone(&root));

        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["0", "2", "1", "6", "5", "4", "3", "#", "#", "#", "#", "#", "#", "#", "#"]
        );
    }

    #[test]
    fn test_unbalanced_cases() {
        /*
                0            0
               / \          / \
              1   2        2   1
             / \      =>      / \
            3   4            4   3
               / \          / \
              5   6        6   5

            level order traversal: 0,1,2,3,4,5,6
        */
        let root = Rc::new(RefCell::new(TreeNode::new("0")));
        let c1 = Rc::new(RefCell::new(TreeNode::new("1")));
        let c2 = Rc::new(RefCell::new(TreeNode::new("2")));
        let c3 = Rc::new(RefCell::new(TreeNode::new("3")));
        let c4 = Rc::new(RefCell::new(TreeNode::new("4")));
        let c5 = Rc::new(RefCell::new(TreeNode::new("5")));
        let c6 = Rc::new(RefCell::new(TreeNode::new("6")));

        root.borrow_mut().attach_left(c1.clone());
        root.borrow_mut().attach_right(c2.clone());
        c1.borrow_mut().attach_left(c3.clone());
        c1.borrow_mut().attach_right(c4.clone());
        c4.borrow_mut().attach_left(c5.clone());
        c4.borrow_mut().attach_right(c6.clone());

        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["0", "1", "2", "3", "4", "#", "#", "#", "#", "5", "6", "#", "#", "#", "#"]
        );

        TreeNode::invert(Rc::clone(&root));

        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["0", "2", "1", "#", "#", "4", "3", "6", "5", "#", "#", "#", "#", "#", "#"]
        );
    }

    #[test]
    fn test_right_children() {
        let root = Rc::new(RefCell::new(TreeNode::new("I")));
        let c1 = Rc::new(RefCell::new(TreeNode::new("love")));
        let c2 = Rc::new(RefCell::new(TreeNode::new("coding")));
        let c3 = Rc::new(RefCell::new(TreeNode::new("in")));
        let c4 = Rc::new(RefCell::new(TreeNode::new("Rust!")));

        root.borrow_mut().attach_right(c1.clone());
        c1.borrow_mut().attach_right(c2.clone());
        c2.borrow_mut().attach_right(c3.clone());
        c3.borrow_mut().attach_right(c4.clone());

        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["I", "#", "love", "#", "coding", "#", "in", "#", "Rust!", "#", "#"]
        );

        TreeNode::invert(Rc::clone(&root));

        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["I", "love", "#", "coding", "#", "in", "#", "Rust!", "#", "#", "#"]
        );
    }

    #[test]
    fn test_edge_cases() {
        let root = Rc::new(RefCell::new(TreeNode::new("0")));
        TreeNode::invert(Rc::clone(&root));
        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["0", "#", "#"]
        );

        let root = Rc::new(RefCell::new(TreeNode::new("0")));
        let left1 = Rc::new(RefCell::new(TreeNode::new("1")));
        root.borrow_mut().attach_left(left1.clone());

        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["0", "1", "#", "#", "#"]
        );
        TreeNode::invert(Rc::clone(&root));
        assert_eq!(
            TreeNode::traverse_level_order(Rc::clone(&root)),
            ["0", "#", "1", "#", "#"]
        );
    }
}

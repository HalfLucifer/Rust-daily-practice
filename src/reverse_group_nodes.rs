use std::cell::RefCell;
use std::rc::Rc;

pub struct SinglyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    successor: Option<Rc<RefCell<Node>>>,
    length: usize,
}

struct Node {
    elem: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            successor: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, elem: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
        }));

        match self.head.take() {
            Some(node) => {
                new_node.borrow_mut().next = Some(Rc::clone(&node));
                self.head = Some(Rc::clone(&new_node));
            }

            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }

        self.length += 1;
    }

    pub fn traverse(&self) -> Vec<i32> {
        fn traverse_recursive(curr: &Option<Rc<RefCell<Node>>>, result: &mut Vec<i32>) {
            if let Some(node) = curr {
                result.push(node.borrow().elem);

                traverse_recursive(&node.borrow().next, result);
            }
        }

        let mut result = vec![];
        traverse_recursive(&self.head, &mut result);
        result
    }

    pub fn reverse_group(&mut self, n: usize) {
        if n > 1 {
            if let Some(node) = &self.head {
                self.head = Some(self.reverse_group_recursively(Rc::clone(&node), n));
            }
        }
    }

    fn reverse_group_recursively(
        &mut self,
        head: Rc<RefCell<Node>>,
        n: usize,
    ) -> Rc<RefCell<Node>> {
        let current_length = self.length;
        let target_node = self.traverse_node(Rc::clone(&head), n);

        // Check whether it can traverse to N-th node
        if let Some(next) = target_node {
            // Reverse front N-th nodes
            let new_head = self.reverse_front_recursively(Rc::clone(&head), n);

            // Attach tail of current nodes to head of next nodes
            let next_head = self.reverse_group_recursively(next, n);
            Rc::clone(&head).borrow_mut().next = Some(next_head);

            return new_head;
        } else {
            // [TODO] refactor to remove the special case

            // Special case: when traversing to (N+1)th node and all nodes fit in groups
            if self.length == 0 && current_length == n {
                // Reverse the last N nodes
                let new_head = self.reverse_front_recursively(Rc::clone(&head), n);
                return new_head;
            }
        }

        // Base case: node count is less than n, return old head
        return head;
    }

    fn traverse_node(&mut self, head: Rc<RefCell<Node>>, pos: usize) -> Option<Rc<RefCell<Node>>> {
        self.traverse_node_recursively(&Some(head), pos)
    }

    fn traverse_node_recursively(
        &mut self,
        head: &Option<Rc<RefCell<Node>>>,
        pos: usize,
    ) -> Option<Rc<RefCell<Node>>> {
        if let Some(node) = head {
            if pos == 0 {
                return Some(Rc::clone(&node));
            } else {
                self.length -= 1;
                return self.traverse_node_recursively(&node.borrow().next, pos - 1);
            }
        }

        None
    }

    fn reverse_front_recursively(
        &mut self,
        head: Rc<RefCell<Node>>,
        n: usize,
    ) -> Rc<RefCell<Node>> {
        // Set new_head default to head
        let mut new_head = Rc::clone(&head);

        // Reached the nth node, memo the head.next as successor
        if n == 1 {
            self.successor = new_head.borrow_mut().next.take();
            return new_head;
        }

        // If head.next is existed
        if let Some(node) = &head.borrow_mut().next {
            // Get new_head recursively by head.next
            new_head = self.reverse_front_recursively(Rc::clone(node), n - 1);

            // Point head.next.next to head
            node.borrow_mut().next = Some(Rc::clone(&head));
        }

        if let Some(node) = &self.successor {
            // Point head.next to successor
            head.borrow_mut().next = Some(Rc::clone(node));
        } else {
            // Point to None if successor does not exist
            head.borrow_mut().next = None;
        }

        // Return the last node as new head
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_group(2);
        assert_eq!(s.traverse(), [1, 0, 3, 2, 5, 4, 7, 6, 9, 8]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_group(3);
        assert_eq!(s.traverse(), [2, 1, 0, 5, 4, 3, 8, 7, 6, 9]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_group(4);
        assert_eq!(s.traverse(), [3, 2, 1, 0, 7, 6, 5, 4, 8, 9]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_group(9);
        assert_eq!(s.traverse(), [8, 7, 6, 5, 4, 3, 2, 1, 0, 9]);
        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_group(10);
        assert_eq!(s.traverse(), [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_edge_cases() {
        let mut s = SinglyLinkedList::new();
        s.reverse_group(1);
        assert_eq!(s.traverse(), []);

        let mut s = SinglyLinkedList::new();
        s.push_front(0);
        s.reverse_group(2);
        assert_eq!(s.traverse(), [0]);

        let mut s = SinglyLinkedList::new();
        s.push_front(0);
        s.push_front(1);
        assert_eq!(s.traverse(), [1, 0]);

        s.reverse_group(2);
        assert_eq!(s.traverse(), [0, 1]);
    }
}

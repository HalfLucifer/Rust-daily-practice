use std::cell::RefCell;
use std::rc::Rc;

pub struct SinglyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    successor: Option<Rc<RefCell<Node>>>,
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

    pub fn reverse_front(&mut self, n: usize) {
        if n > 1 {
            if let Some(node) = &self.head {
                self.head = Some(self.reverse_front_recursively(Rc::clone(&node), n));
            }
        }
    }

    pub fn reverse_range(&mut self, m: usize, n: usize) {
        assert!(m > 0 && n > 0 && m <= n);

        if m == 1 {
            self.reverse_front(n);
            return;
        }

        let mut rev_head = None;

        // Get the m-th node
        if let Some(node) = self.get_node(m) {
            // Reverse m to n nodes
            rev_head = Some(self.reverse_front_recursively(node, n - m + 1));
        }

        if m == 1 {
            // Point to head of reverse list
            if let Some(node) = &self.head {
                node.borrow_mut().next = rev_head;
            }
        } else {
            // Get the (m-1)-th node
            if let Some(node) = self.get_node(m - 1) {
                node.borrow_mut().next = rev_head;
            }
        }
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

    fn get_node(&self, n: usize) -> Option<Rc<RefCell<Node>>> {
        fn get_node_recursively(
            head: &Option<Rc<RefCell<Node>>>,
            n: usize,
        ) -> Option<Rc<RefCell<Node>>> {
            if let Some(node) = head {
                if n == 1 {
                    return Some(Rc::clone(&node));
                } else {
                    return get_node_recursively(&node.borrow().next, n - 1);
                }
            }

            None
        }

        get_node_recursively(&self.head, n)
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

        s.reverse_range(4, 5);
        assert_eq!(s.traverse(), [0, 1, 2, 4, 3, 5, 6, 7, 8, 9]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_range(3, 7);
        assert_eq!(s.traverse(), [0, 1, 6, 5, 4, 3, 2, 7, 8, 9]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_range(2, 9);
        assert_eq!(s.traverse(), [0, 8, 7, 6, 5, 4, 3, 2, 1, 9]);
    }

    #[test]
    fn test_edge_cases() {
        let mut s = SinglyLinkedList::new();
        s.reverse_range(1, 1);
        assert_eq!(s.traverse(), []);

        let mut s = SinglyLinkedList::new();
        s.push_front(0);
        s.reverse_range(1, 1);
        assert_eq!(s.traverse(), [0]);

        let mut s = SinglyLinkedList::new();
        s.push_front(0);
        s.push_front(1);
        assert_eq!(s.traverse(), [1, 0]);

        s.reverse_range(1, 2);
        assert_eq!(s.traverse(), [0, 1]);
    }

    #[test]
    fn test_bound_cases() {
        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_range(1, 5);
        assert_eq!(s.traverse(), [4, 3, 2, 1, 0, 5, 6, 7, 8, 9]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_range(5, 10);
        assert_eq!(s.traverse(), [0, 1, 2, 3, 9, 8, 7, 6, 5, 4]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.reverse_range(1, 10);
        assert_eq!(s.traverse(), [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }
}

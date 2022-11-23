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
        for j in 1..10 {
            let mut s = SinglyLinkedList::new();
            let mut expected = vec![];

            for i in (1..10).rev() {
                s.push_front(i);
            }

            for k in (1..=j).rev() {
                expected.push(k as i32);
            }

            for k in j + 1..10 {
                expected.push(k as i32);
            }

            s.reverse_front(j);
            assert_eq!(s.traverse(), expected);
        }
    }

    #[test]
    fn test_edge_cases() {
        let mut s = SinglyLinkedList::new();
        s.reverse_front(0);
        assert_eq!(s.traverse(), []);

        let mut s = SinglyLinkedList::new();
        s.push_front(0);
        s.reverse_front(1);
        assert_eq!(s.traverse(), [0]);

        let mut s = SinglyLinkedList::new();
        for i in 0..5 {
            s.push_front(i);
        }

        let mut expected = s.traverse();
        expected.sort();
        s.reverse_front(10);
        assert_eq!(s.traverse(), expected);
    }

    #[test]
    fn test_double_reverse_cases() {
        let mut s = SinglyLinkedList::new();

        for i in 1..100 {
            if i == 99 {
                s.push_front(1);
            } else {
                s.push_front(0);
            }
        }

        let expected = s.traverse().clone();
        s.reverse_front(99);
        s.reverse_front(99);
        assert_eq!(s.traverse(), expected);
    }
}

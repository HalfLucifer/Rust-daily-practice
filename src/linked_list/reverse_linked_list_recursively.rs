use std::cell::RefCell;
use std::rc::Rc;

pub struct SinglyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
}

struct Node {
    elem: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        Self { head: None }
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

    pub fn reverse(&mut self) {
        fn reverse_recursively(head: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
            // Set new_head default to head
            let mut new_head = Rc::clone(&head);

            // If head.next is existed
            if let Some(node) = &head.borrow_mut().next {
                // Get new_head recursively by head.next
                new_head = reverse_recursively(Rc::clone(node));

                // Point head.next.next to head
                node.borrow_mut().next = Some(Rc::clone(&head));
            }

            // Point head.next to None
            head.borrow_mut().next = None;

            // Return the tail node for new_head
            new_head
        }

        if let Some(node) = &self.head {
            self.head = Some(reverse_recursively(Rc::clone(&node)));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut s = SinglyLinkedList::new();

        for i in 1..10 {
            s.push_front(i);
        }

        s.reverse();
        assert_eq!(s.traverse(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_edge_cases() {
        let mut s = SinglyLinkedList::new();
        s.reverse();
        assert_eq!(s.traverse(), []);

        s.push_front(0);
        assert_eq!(s.traverse(), [0]);

        let mut s = SinglyLinkedList::new();
        s.push_front(i32::MIN);
        s.push_front(0);
        s.push_front(i32::MAX);
        s.reverse();
        assert_eq!(s.traverse(), [i32::MIN, 0, i32::MAX]);
    }

    #[test]
    fn test_zeros_and_ones_cases() {
        let mut s = SinglyLinkedList::new();
        let mut expected = vec![];

        for i in 1..100 {
            if i % 2 == 0 {
                s.push_front(0);
                expected.push(0);
            } else {
                s.push_front(1);
                expected.push(1);
            }
        }

        s.reverse();
        assert_eq!(s.traverse(), expected);
    }

    #[test]
    fn test_only_one_cases() {
        let mut s = SinglyLinkedList::new();

        for i in 1..100 {
            if i == 99 {
                s.push_front(1);
            } else {
                s.push_front(0);
            }
        }

        let mut expected = s.traverse();
        expected.sort();
        s.reverse();
        assert_eq!(s.traverse(), expected);
    }
}

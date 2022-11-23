use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Default + Copy + PartialEq> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(node) => {
                new_node.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&new_node));
                self.head = Some(Rc::clone(&new_node));
            }

            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            let ret = head.borrow().elem;

            if let Some(next_node) = head.borrow_mut().next.take() {
                next_node.borrow_mut().prev = None;
                self.head = Some(next_node);
            } else {
                self.head = None;
                self.tail = None;
            }

            return Some(ret);
        }

        None
    }

    pub fn insert_front(&mut self, node: Rc<RefCell<Node<T>>>) {
        match self.head.take() {
            Some(old_head) => {
                node.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::clone(&node));
                self.head = Some(Rc::clone(&node));
            }

            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }

    pub fn remove(&mut self, node: Rc<RefCell<Node<T>>>) {
        let prev_node = node.borrow_mut().prev.take();
        let next_node = node.borrow_mut().next.take();

        match (prev_node, next_node) {
            (Some(prev), Some(next)) => {
                // Connecting previous node with next node
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;

                prev.borrow_mut().next = Some(Rc::clone(&next));
                next.borrow_mut().prev = Some(Rc::clone(&prev));
            }

            (None, Some(next)) => {
                // Node-to-remove is head, update to next node
                next.borrow_mut().prev = None;
                node.borrow_mut().next = None;

                self.head = Some(next);
            }

            (Some(prev), None) => {
                // Node-to-remove is tail, update to previous node
                prev.borrow_mut().next = None;
                node.borrow_mut().prev = None;

                self.tail = Some(prev);
            }

            _ => {
                // Node-to-remove is the only node
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;

                self.head = None;
                self.tail = None;
            }
        }
    }

    pub fn traverse(&self) -> Vec<T> {
        fn traverse_recursive<T: Copy>(curr: &Option<Rc<RefCell<Node<T>>>>, result: &mut Vec<T>) {
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

struct Node<T> {
    elem: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            elem: value,
            next: None,
            prev: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut d: DoublyLinkedList<i32> = DoublyLinkedList::new();

        for i in 0..10 {
            d.push_front(i);
        }
        assert_eq!(d.traverse(), (0..10).rev().collect::<Vec<_>>());

        for i in (0..10).rev() {
            assert_eq!(d.pop_front(), Some(i));
        }

        assert_eq!(d.pop_front(), None);
        assert_eq!(d.traverse(), vec![]);
    }

    #[test]
    fn test_edge_cases() {
        let mut d: DoublyLinkedList<i32> = DoublyLinkedList::new();

        assert_eq!(d.traverse(), vec![]);
        assert_eq!(d.pop_front(), None);

        d.push_front(i32::MIN);
        assert_eq!(d.traverse(), vec![i32::MIN]);

        d.pop_front();
        d.push_front(i32::MAX);
        assert_eq!(d.traverse(), vec![i32::MAX]);
    }

    #[test]
    fn test_remove_cases() {
        let mut d: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let mid_node = Rc::new(RefCell::new(Node::new(0)));
        let min_node = Rc::new(RefCell::new(Node::new(i32::MIN)));
        let max_node = Rc::new(RefCell::new(Node::new(i32::MAX)));

        d.insert_front(Rc::clone(&max_node));
        d.insert_front(Rc::clone(&mid_node));
        d.insert_front(Rc::clone(&min_node));
        assert_eq!(d.traverse(), vec![i32::MIN, 0, i32::MAX]);

        d.remove(Rc::clone(&max_node));
        assert_eq!(d.traverse(), vec![i32::MIN, 0]);

        d.remove(Rc::clone(&min_node));
        assert_eq!(d.traverse(), vec![0]);

        d.remove(Rc::clone(&mid_node));
        assert_eq!(d.traverse(), vec![]);
    }
}

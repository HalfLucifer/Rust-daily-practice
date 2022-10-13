use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct LinkedHashMap<T> {
    map: HashMap<T, Rc<RefCell<Node<T>>>>,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Default + Copy + Eq + Hash> LinkedHashMap<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn insert(&mut self, key: T, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            key: key,
            value: value,
            next: None,
            prev: None,
        }));

        self.map.insert(key, Rc::clone(&new_node));

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

    pub fn remove(&mut self, key: T) -> Option<Rc<RefCell<Node<T>>>> {
        if let Some(node) = self.map.get(&key) {
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

            return self.map.remove(&key);
        }

        None
    }

    pub fn remove_last(&mut self) -> Option<(T, T)> {
        if let Some(tail) = self.tail.take() {
            if let Some(prev_node) = tail.borrow_mut().prev.take() {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_node);
            } else {
                self.head = None;
                self.tail = None;
            }

            self.map.remove(&tail.borrow().key);

            Some((tail.borrow().key, tail.borrow().value))
        } else {
            None
        }
    }

    pub fn get(&self, key: T) -> Option<T> {
        if let Some(node) = self.map.get(&key) {
            Some(node.borrow().value)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: T) -> bool {
        self.map.contains_key(&key)
    }

    pub fn traverse(&self) -> Vec<(T, T)> {
        let mut result = vec![];

        fn traverse_recursive<T: Copy>(
            curr: &Option<Rc<RefCell<Node<T>>>>,
            result: &mut Vec<(T, T)>,
        ) {
            if let Some(node) = curr {
                let key = node.borrow().key;
                let value = node.borrow().value;
                result.push((key, value));

                traverse_recursive(&node.borrow().next, result);
            }
        }

        traverse_recursive(&self.head, &mut result);
        result
    }
}

struct Node<T> {
    key: T,
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(key: T, value: T) -> Self {
        Self {
            key: key,
            value: value,
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
        let mut lhm = LinkedHashMap::new();

        for i in 0..10 {
            lhm.insert(i, i);
        }

        assert_eq!(
            lhm.traverse(),
            (0..10).rev().map(|v| (v, v)).collect::<Vec<_>>()
        );

        for i in 0..10 {
            assert!(lhm.contains_key(i));
        }

        for i in 0..10 {
            lhm.remove_last();
            assert_eq!(
                lhm.traverse(),
                (i + 1..10).rev().map(|v| (v, v)).collect::<Vec<_>>()
            );
        }

        assert_eq!(lhm.traverse(), []);
    }

    #[test]
    fn test_edge_cases() {
        let mut lhm = LinkedHashMap::new();

        lhm.insert(0, 0);
        assert!(lhm.contains_key(0));
        assert_eq!(lhm.traverse(), [(0, 0)]);

        lhm.remove_last();
        assert_eq!(lhm.traverse(), []);
        assert_eq!(lhm.remove_last(), None);
    }

    #[test]
    fn test_removal_cases() {
        let mut lhm = LinkedHashMap::new();
        let mut expected = (0..10).rev().map(|v| (v, v)).collect::<Vec<_>>();
        for i in 0..10 {
            lhm.insert(i, i);
            assert!(lhm.contains_key(i));
        }

        lhm.remove(0);
        expected.remove(9);
        assert_eq!(lhm.traverse(), expected);

        lhm.remove(9);
        expected.remove(0);
        assert_eq!(lhm.traverse(), expected);

        expected.remove(4);
        lhm.remove(4);
        assert_eq!(lhm.traverse(), expected);
    }
}

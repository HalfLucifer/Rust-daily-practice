use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/*
    Least Recently Used Cache

       Use LinkedHashMap to achieve constant time operation
       - put(): time complexity O(1)
       - get(): time complexity O(1)
*/
pub struct LRUCache<T> {
    capacity: usize,
    cache: LinkedHashMap<T>,
}

impl<T: Default + Copy + Eq + Hash> LRUCache<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            cache: LinkedHashMap::new(),
        }
    }

    pub fn put(&mut self, key: T, value: T) {
        // Check if key is existed in cache
        if self.cache.contains_key(key) {
            // If it exists, update the value & make it recent
            self.cache.update(key, value);
            self.make_recent(key);
        } else {
            // If capacity is full, remove the least recently used node
            if self.cache.len() >= self.capacity {
                self.cache.remove_last();
            }

            // Key is absent, insert a new node
            if self.capacity > 0 {
                self.cache.insert(key, value);
            }
        }
    }

    pub fn get(&mut self, key: T) -> Option<T> {
        if self.cache.contains_key(key) {
            self.make_recent(key);
            return self.cache.get(key);
        }

        None
    }

    pub fn traverse(&self) -> Vec<(T, T)> {
        self.cache.traverse()
    }

    fn make_recent(&mut self, key: T) {
        if let Some(node) = self.cache.remove(key) {
            self.cache.insert_node(key, node);
        }
    }
}

/*
   Node - for doubly linked list
*/
struct Node<T> {
    key: T,
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

/*
   LinkedHashMap = HashMap + Doubly Linked List
*/
struct LinkedHashMap<T> {
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

        self.insert_node(key, new_node);
    }

    pub fn insert_node(&mut self, key: T, new_node: Rc<RefCell<Node<T>>>) {
        self.map.insert(key, Rc::clone(&new_node));

        // Insert a new node to head
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
        // Remove a node from tail
        if let Some(tail) = self.tail.take() {
            if let Some(prev_node) = tail.borrow_mut().prev.take() {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_node);
            } else {
                self.head = None;
                self.tail = None;
            }

            self.map.remove(&tail.borrow().key);
            return Some((tail.borrow().key, tail.borrow().value));
        }

        None
    }

    pub fn update(&mut self, key: T, value: T) {
        if let Some(node) = self.map.get(&key) {
            node.borrow_mut().value = value;
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

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn traverse(&self) -> Vec<(T, T)> {
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
        let mut lru: LRUCache<i32> = LRUCache::new(3);
        lru.put(0, 0);
        lru.put(1, -1);
        lru.put(2, -2);
        assert_eq!(lru.traverse(), [(2, -2), (1, -1), (0, 0)]);

        for i in 0..3 {
            assert_eq!(lru.get(i), Some(-i));
        }

        lru.get(0);
        assert_eq!(lru.traverse(), [(0, 0), (2, -2), (1, -1)]);

        lru.get(1);
        assert_eq!(lru.traverse(), [(1, -1), (0, 0), (2, -2)]);

        lru.put(4, -4);
        assert_eq!(lru.traverse(), [(4, -4), (1, -1), (0, 0)]);
    }

    #[test]
    fn test_edge_cases() {
        let mut lru: LRUCache<i32> = LRUCache::new(0);
        lru.put(0, 0);
        assert_eq!(lru.get(0), None);
        assert_eq!(lru.traverse(), []);

        let mut lru: LRUCache<i32> = LRUCache::new(1);
        lru.put(i32::MIN, i32::MAX);
        assert_eq!(lru.get(i32::MIN), Some(i32::MAX));
        assert_eq!(lru.traverse(), [(i32::MIN, i32::MAX)]);

        lru.put(i32::MAX, i32::MIN);
        assert_eq!(lru.get(i32::MIN), None);
        assert_eq!(lru.traverse(), [(i32::MAX, i32::MIN)]);
    }

    #[test]
    fn test_cache_full_cases() {
        let mut lru: LRUCache<i32> = LRUCache::new(10);

        for i in 1..100 {
            lru.put(i, i);
        }

        assert_eq!(
            lru.traverse(),
            (90..100).rev().map(|v| (v, v)).collect::<Vec<_>>()
        );

        for i in (90..100).rev() {
            lru.get(i);
        }

        assert_eq!(
            lru.traverse(),
            (90..100).map(|v| (v, v)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_update_value_cases() {
        let mut lru: LRUCache<i32> = LRUCache::new(10);

        for i in 0..10 {
            lru.put(i, i);
        }

        assert_eq!(
            lru.traverse(),
            (0..10).rev().map(|v| (v, v)).collect::<Vec<_>>()
        );

        for i in (0..10).rev() {
            lru.put(i, i.pow(2));
        }

        assert_eq!(
            lru.traverse(),
            (0..10).map(|v| (v, i32::pow(v, 2))).collect::<Vec<_>>()
        );
    }
}

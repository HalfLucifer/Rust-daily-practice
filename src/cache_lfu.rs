use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/*
    Least Frequently Used Cache

       Use LinkedHashMap to achieve constant time operation
       - put(): time complexity O(1)
       - get(): time complexity O(1)
*/
pub struct LFUCache<T> {
    capacity: usize,
    min_frequency: usize,
    key_to_value: HashMap<T, T>,
    key_to_freq: HashMap<T, usize>,
    freq_to_key: HashMap<usize, LinkedHashMap<T>>,
}

impl<T: Default + Copy + Eq + Hash + Ord> LFUCache<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        Self {
            capacity: capacity,
            min_frequency: 0,
            key_to_value: HashMap::new(),
            key_to_freq: HashMap::new(),
            freq_to_key: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: T, value: T) {
        // Check if key is existed in cache
        if self.key_to_value.contains_key(&key) {
            // If it exists, add the frequency
            self.key_to_value.insert(key, value);
            self.increase_frequency(key, 1);
            return;
        }

        // If capacity is full, remove the least frequently used one
        if self.key_to_value.len() >= self.capacity {
            self.remove_least_frequently_used_key();
        }

        // Key is absent, insert a new value to HashMaps
        self.key_to_value.insert(key, value);
        self.key_to_freq.insert(key, 1);
        self.min_frequency = 1;

        // Insert the key to freq-to-key
        // NOTE: may consider to replace LinkedHashMap with LinkedHashSet
        self.freq_to_key.entry(1).or_default().insert(key, key);
    }

    pub fn get(&mut self, key: T) -> Option<&T> {
        if !self.key_to_value.contains_key(&key) {
            return None;
        }

        self.increase_frequency(key, 1);

        return self.key_to_value.get(&key);
    }

    fn increase_frequency(&mut self, key: T, freq: usize) {
        if let Some(old_freq) = self.key_to_freq.get(&key) {
            let new_freq = old_freq + freq;

            // Remove old entry from freq-to-key
            if let Some(hm) = self.freq_to_key.get_mut(&old_freq) {
                hm.remove(key);

                // Update min frequency if there is nothing left
                if hm.is_empty() {
                    self.freq_to_key.remove(&old_freq);

                    if self.min_frequency == *old_freq {
                        self.min_frequency = new_freq;
                    }
                }
            }

            // Insert the key to freq-to-key
            // NOTE: may consider to replace LinkedHashMap with LinkedHashSet
            self.freq_to_key
                .entry(new_freq)
                .or_default()
                .insert(key, key);

            // Update key-to-freq Hashmap
            self.key_to_freq.insert(key, new_freq);
        }
    }

    fn remove_least_frequently_used_key(&mut self) {
        if self.min_frequency == 0 {
            return;
        }

        if let Some(hm) = self.freq_to_key.get_mut(&self.min_frequency) {
            // Remove the oldest entry from LinkedHashMap
            if let Some((k, _)) = hm.remove_last() {
                // Remove entry from Hashmaps
                self.key_to_value.remove(&k);
                self.key_to_freq.remove(&k);
            }
        }
    }

    pub fn traverse_key_to_value(&self) -> Vec<(T, T)> {
        let mut result = self
            .key_to_value
            .iter()
            .map(|k| (*k.0, *k.1))
            .collect::<Vec<(_, _)>>();
        result.sort();
        result
    }

    pub fn traverse_key_to_freq(&self) -> Vec<(T, usize)> {
        let mut result = self
            .key_to_value
            .keys()
            .map(|k| (*k, *self.key_to_freq.get(k).unwrap()))
            .collect::<Vec<(_, _)>>();
        result.sort();
        result
    }

    pub fn traverse_freq_to_key(&self) -> HashMap<usize, Vec<(T, T)>> {
        self.freq_to_key
            .keys()
            .map(|k| (*k, self.freq_to_key.get(k).unwrap().traverse()))
            .collect::<HashMap<_, _>>()
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
#[derive(Default)]
struct LinkedHashMap<T> {
    map: HashMap<T, Rc<RefCell<Node<T>>>>,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Eq + Hash + Copy> LinkedHashMap<T> {
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

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut f: LFUCache<i32> = LFUCache::new(3);

        f.put(1, 1);
        f.put(2, 2);
        f.put(3, 3);
        assert_eq!(f.traverse_key_to_value(), vec![(1, 1), (2, 2), (3, 3)]);

        for i in 4..100 {
            f.put(i, i);
            assert_eq!(
                f.traverse_key_to_value(),
                (i - 2..=i).map(|n| (n, n)).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        let mut f: LFUCache<i32> = LFUCache::new(1);
        f.put(0, 0);

        for i in 1..100 {
            f.put(i, i);
            assert_eq!(f.traverse_key_to_value(), vec![(i, i)]);
        }

        f.put(0, 1);
        f.put(0, 2);
        f.put(0, 3);

        assert_eq!(f.traverse_key_to_value(), vec![(0, 3)]);
        assert_eq!(f.traverse_key_to_freq(), vec![(0, 3)]);

        for i in 1..100 {
            f.put(i, i);
            assert_eq!(f.traverse_key_to_value(), vec![(i, i)]);
        }
    }

    #[test]
    fn test_cache_full_cases() {
        let mut f: LFUCache<i32> = LFUCache::new(3);
        f.put(0, 0);
        f.put(1, 1);
        f.put(2, 2);

        f.get(0);
        f.get(1);
        f.get(2);

        f.get(0);
        f.get(1);
        f.get(2);

        assert_eq!(f.traverse_key_to_freq(), vec![(0, 3), (1, 3), (2, 3)]);

        f.put(100, 100);

        assert_eq!(f.traverse_key_to_value(), vec![(1, 1), (2, 2), (100, 100)]);
        assert_eq!(f.traverse_key_to_freq(), vec![(1, 3), (2, 3), (100, 1)]);

        f.get(100);
        f.get(100);
        f.get(1);
        f.put(200, 200);

        assert_eq!(
            f.traverse_key_to_value(),
            vec![(1, 1), (100, 100), (200, 200)]
        );
        assert_eq!(f.traverse_key_to_freq(), vec![(1, 4), (100, 3), (200, 1)]);

        f.get(200);
        f.get(200);
        f.put(300, 300);

        assert_eq!(
            f.traverse_key_to_value(),
            vec![(1, 1), (200, 200), (300, 300)]
        );
        assert_eq!(f.traverse_key_to_freq(), vec![(1, 4), (200, 3), (300, 1)]);
    }
}

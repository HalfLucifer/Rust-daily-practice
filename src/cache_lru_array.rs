use std::collections::HashMap;

/*
    Least Recently Used Cache - array version

    Use arrays to replace doubly linked list for achieving O(1) time complexity
    - Push new element to tail
    - Pop LRU element from head
*/
pub struct LRUCache {
    capacity: usize,
    key_to_index: HashMap<i32, usize>,
    keys: Vec<i32>,
    values: Vec<i32>,
    prev: Vec<usize>,
    next: Vec<usize>,
    head: usize,
    tail: usize,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            key_to_index: HashMap::new(),
            keys: vec![0; capacity],
            values: vec![0; capacity],
            prev: vec![usize::MAX; capacity], // initialize to MAX for link validation
            next: vec![usize::MAX; capacity], // initialize to MAX for link validation
            head: 0,
            tail: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&index) = self.key_to_index.get(&key) {
            self.update_links(index);
            return self.values[index];
        }

        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // Key/value is already existed
        if let Some(&index) = self.key_to_index.get(&key) {
            self.values[index] = value;
            self.update_links(index);
            return;
        }

        let mut index = self.key_to_index.len();

        // Capacity is full
        if self.key_to_index.len() == self.capacity {
            // Remove LRU key/value element
            index = self.head;
            self.key_to_index.remove(&self.keys[index]);
        }

        // Update key/value element
        self.key_to_index.insert(key, index);
        self.keys[index] = key;
        self.values[index] = value;

        self.update_links(index);
    }

    // Remove the element from list and push it to tail
    fn update_links(&mut self, index: usize) {
        // If index is tail, no need for action
        if self.tail == index {
            return;
        }

        if self.head == index {
            // If index is head (LRU), remove it by updating to previous link
            self.head = self.prev[index];
        } else {
            // If index is in middle of list, remove it by
            //   1. point prev.next to next
            //   2. point next.prev to prev
            let pre = self.prev[index];
            let nxt = self.next[index];

            // No need to update if link is not existed
            if let Some(elem) = self.next.get_mut(pre) {
                *elem = nxt;
            }
            // No need to update if link is not existed
            if let Some(elem) = self.prev.get_mut(nxt) {
                *elem = pre;
            }
        }

        // Push index to tail
        self.prev[self.tail] = index;
        self.next[index] = self.tail;
        self.tail = index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut lru = LRUCache::new(3);
        lru.put(1, 1);
        lru.put(2, 2);
        lru.put(3, 3);

        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(2), 2);
        assert_eq!(lru.get(1), 1);

        lru.put(4, 4);
        lru.put(5, 5);

        assert_eq!(lru.get(3), -1);
        assert_eq!(lru.get(2), -1);
        assert_eq!(lru.get(1), 1);

        lru.put(6, 6);

        assert_eq!(lru.get(6), 6);
        assert_eq!(lru.get(5), 5);
        assert_eq!(lru.get(4), -1);
        assert_eq!(lru.get(1), 1);
    }

    #[test]
    fn test_edge_cases() {
        let mut lru = LRUCache::new(1);
        lru.put(0, 0);

        assert_eq!(lru.get(0), 0);
        assert_eq!(lru.get(1), -1);

        lru.put(-1, -1);
        assert_eq!(lru.get(-1), -1);
    }
}

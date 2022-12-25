use std::collections::HashMap;

/*
   Max priority queue
*/
pub struct PriorityQueue<T> {
    queue: Vec<T>,
    map: HashMap<T, i32>,
}

impl<T: Clone + PartialEq + Eq + std::hash::Hash> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: vec![],
            map: HashMap::new(),
        }
    }

    pub fn push(&mut self, value: T, priority: i32) {
        self.map.insert(value.clone(), priority);
        self.queue.push(value);
        self.sift_up();
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            return None;
        }

        let len = self.queue.len();
        self.queue.swap(0, len - 1);

        if let Some(value) = self.queue.pop() {
            self.map.remove(&value);
            self.sift_down(0);

            Some(value)
        } else {
            None
        }
    }

    pub fn remove(&mut self, value: T) {
        self.map.remove(&value);

        if let Some(pos) = self.queue.iter().position(|k| *k == value) {
            let len = self.queue.len();
            self.queue.swap(pos, len - 1);
            self.queue.pop();
            self.sift_down(pos);
        }
    }

    pub fn modify(&mut self, value: T, priority: i32) {
        self.map.insert(value.clone(), priority);

        if let Some(pos) = self.queue.iter().position(|k| *k == value) {
            self.queue.remove(pos);
            self.queue.insert(0, value);
            self.sift_down(0);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn get_priority(&self, value: &T) -> Option<&i32> {
        self.map.get(&value)
    }

    fn is_greater(&self, index_a: usize, index_b: usize) -> bool {
        let v1 = &self.queue[index_a];
        let v2 = &self.queue[index_b];

        match (self.get_priority(v1), self.get_priority(v2)) {
            (Some(a), Some(b)) => a > b,
            _ => false,
        }
    }

    fn sift_up(&mut self) {
        let mut index = self.queue.len() as i32 - 1;
        let mut parent = (index - 1) / 2;

        while parent >= 0 && self.is_greater(index as usize, parent as usize) {
            self.queue.swap(index as usize, parent as usize);
            index = parent;
            parent = (index - 1) / 2;
        }
    }

    fn sift_down(&mut self, index: usize) {
        let left_child = index * 2 + 1;
        let right_child = index * 2 + 2;
        let mut smaller_index = index;

        // Find the index of smaller children
        if left_child < self.queue.len() && self.is_greater(left_child, smaller_index) {
            smaller_index = left_child;
        }

        if right_child < self.queue.len() && self.is_greater(right_child, smaller_index) {
            smaller_index = right_child;
        }

        if index != smaller_index {
            self.queue.swap(index, smaller_index);
            self.sift_down(smaller_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();

        (0..100).for_each(|v| {
            pq.push(v, v);
        });

        (0..100).rev().for_each(|v| {
            assert_eq!(pq.pop(), Some(v));
        });

        (0..100).rev().for_each(|v| {
            pq.push(v, v);
        });

        (0..100).rev().for_each(|v| {
            assert_eq!(pq.pop(), Some(v));
        });
    }

    #[test]
    fn test_push_and_pop_cases() {
        let mut pq: PriorityQueue<&str> = PriorityQueue::new();

        pq.push("Apple", 10);
        pq.push("Grape", 5);
        pq.push("Kiwi", 60);
        pq.push("Banana", 8);
        pq.push("Cherry", 50);
        pq.push("Peach", 99);

        let expected = ["Peach", "Kiwi", "Cherry", "Apple", "Banana", "Grape"];
        expected.iter().for_each(|k| {
            assert_eq!(pq.pop(), Some(*k));
        });

        assert_eq!(pq.pop(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn test_modify_cases() {
        let mut pq: PriorityQueue<&str> = PriorityQueue::new();

        pq.push("Apple", 90);
        pq.push("Grape", 75);
        pq.push("Kiwi", 33);
        pq.push("Banana", 20);
        pq.push("Cherry", 10);
        pq.push("Peach", 1);

        let expected = ["Apple", "Grape", "Kiwi", "Banana", "Cherry", "Peach"];
        expected.iter().for_each(|k| {
            assert_eq!(pq.pop(), Some(*k));
        });
    }

    #[test]
    fn test_remove_cases() {
        let mut pq: PriorityQueue<&str> = PriorityQueue::new();

        pq.push("Apple", 10);
        pq.push("Grape", 5);
        pq.push("Kiwi", 60);
        pq.push("Banana", 8);
        pq.push("Cherry", 50);
        pq.push("Peach", 99);

        pq.remove("Grape");
        pq.remove("Peach");
        pq.remove("Apple");

        let expected = ["Kiwi", "Cherry", "Banana"];
        expected.iter().for_each(|k| {
            assert_eq!(pq.pop(), Some(*k));
        });
    }

    #[test]
    fn test_edge_cases() {
        let mut pq: PriorityQueue<&str> = PriorityQueue::new();

        assert_eq!(pq.pop(), None);
        assert_eq!(pq.get_priority(&"Apple"), None);
        assert!(pq.is_empty());

        pq.push("Rust", 0);
        pq.push("C++", 80);
        pq.modify("Rust", 99);
        pq.remove("C++");

        assert_eq!(pq.pop(), Some("Rust"));
    }
}

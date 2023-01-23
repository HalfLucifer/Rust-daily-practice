/*
    Randomized Set
    - insert: O(1)
    - remove: O(1)
    - get_random: O(1)
 */
use rand::Rng;
use std::collections::HashMap;

pub struct RandomizedSet {
    value_to_index: HashMap<i32, usize>,
    index_to_value: HashMap<usize, i32>,
    counter: usize,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            value_to_index: HashMap::new(),
            index_to_value: HashMap::new(),
            counter: 0,
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.value_to_index.contains_key(&val) {
            return false;
        }

        self.value_to_index.insert(val, self.counter);
        self.index_to_value.insert(self.counter, val);
        self.counter += 1;

        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if !self.value_to_index.contains_key(&val) {
            return false;
        }

        let last_index = self.counter - 1;
        let index = self.value_to_index.remove(&val).unwrap();
        self.index_to_value.remove(&index);

        // Update the last value to removed index
        if index != last_index {
            let last_val = self.index_to_value.remove(&last_index).unwrap();
            self.value_to_index.insert(last_val, index);
            self.index_to_value.insert(index, last_val);
        }

        self.counter -= 1;

        true
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..self.counter);
        *self.index_to_value.get(&rand_index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut rs = RandomizedSet::new();
        for i in 0..10 {
            rs.insert(i * 10);
        }

        rs.remove(10);

        let expected = [0, 20, 30, 40, 50, 60, 70, 80, 90];
        for _ in 0..100 {
            let num = rs.get_random();
            assert!(expected.contains(&num));
        }

        rs.remove(30);
        rs.remove(50);
        rs.remove(70);
        rs.remove(90);

        let expected = [0, 20, 40, 60, 80];
        for _ in 0..100 {
            let num = rs.get_random();
            assert!(expected.contains(&num));
        }

        rs.remove(20);
        rs.remove(40);
        rs.remove(60);
        rs.remove(80);

        let expected = [0, 20];
        for _ in 0..100 {
            let num = rs.get_random();
            assert!(expected.contains(&num));
        }
    }

    #[test]
    fn test_edge_cases() {
        let mut rs = RandomizedSet::new();
        rs.insert(0);
        assert_eq!(rs.get_random(), 0);
    }
}

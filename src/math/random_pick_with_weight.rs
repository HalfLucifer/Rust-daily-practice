/*
   Given an array of positive integers w where w[i] describes the weight of the
   ith index, implement pickIndex() which randomly picks an index in the range
   [0, w.length - 1]. The probability of picking an index i is w[i] / sum(w)
*/
use rand::Rng;

pub struct RandomPick {
    prefix_sum: Vec<i32>,
    total_weight: i32,
}

impl RandomPick {
    pub fn new(w: Vec<i32>) -> Self {
        let mut v = vec![0; w.len() + 1];
        let mut sum = 0;

        // Build a prefix sum array
        for i in 0..w.len() {
            v[i + 1] = w[i] + v[i];
            sum += w[i];
        }

        Self {
            prefix_sum: v,
            total_weight: sum,
        }
    }

    pub fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0..self.total_weight);

        // For a sorted array, use binary search for O(logN) time complexity
        let res = self.prefix_sum.binary_search(&rand);

        if res.is_ok() {
            res.ok().unwrap() as i32
        } else {
            res.err().unwrap() as i32 - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let rp = RandomPick::new(vec![1, 3]);

        for _ in 0..10 {
            let i = rp.pick_index();
            assert!(i == 0 || i == 1);
        }

        let rp = RandomPick::new(vec![5, 4, 3, 2, 1]);

        for _ in 0..100 {
            let i = rp.pick_index();
            assert!(i <= 4);
        }
    }

    #[test]
    fn test_edge_cases() {
        let rp = RandomPick::new(vec![1]);

        for _ in 0..100 {
            assert_eq!(rp.pick_index(), 0);
        }
    }
}

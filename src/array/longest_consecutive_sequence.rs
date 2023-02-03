/*
    Given an unsorted array of integers nums, return the length of the longest
    consecutive elements sequence.
*/
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let hs: HashSet<i32> = HashSet::from_iter(nums.into_iter());

    for n in &hs {
        if !hs.contains(&(n - 1)) {
            let mut streak = 1;

            while hs.contains(&(n + streak)) {
                streak += 1;
            }

            res = res.max(streak);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(longest_consecutive(vec![0]), 1);
        assert_eq!(longest_consecutive(vec![0, 1]), 2);
    }
}

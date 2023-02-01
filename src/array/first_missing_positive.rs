/*
    Given an unsorted integer array nums, return the smallest missing positive integer

    Use cyclic sort to achieve
    - Time complexity: O(n)
    - Space complexity: O(1)
*/
pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[i] != nums[nums[i] as usize - 1] {
            let next = nums[i] as usize - 1;
            nums.swap(i, next);
        }
    }

    for i in 0..nums.len() {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }

    nums.len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(first_missing_positive(vec![2, 0]), 1);
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(first_missing_positive(vec![4, 6, 2, 3, 5, 9, 0, 1, 7]), 8);
        assert_eq!(first_missing_positive((0..100).collect::<Vec<i32>>()), 100);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(first_missing_positive(vec![]), 1);
        assert_eq!(first_missing_positive(vec![0]), 1);
    }
}

/*
    Given an integer array nums, find the subarray with the largest sum and return its sum
*/

/*
    Bottom-up DP method
    - Time complexity: O(n)
    - Space complexity: O(n)
*/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // DP table: dp[i] is the largest subarray sum ended at i
    let mut dp = vec![0; nums.len()];
    let mut res = nums[0];

    // Base case: nums[0] is the sum ended at 0
    dp[0] = nums[0];

    for i in 1..nums.len() {
        // DP trasition
        dp[i] = nums[i].max(dp[i - 1] + nums[i]);

        // Update the solution
        res = res.max(dp[i]);
    }

    res
}

/*
    Space optimized DP method
    - Time complexity: O(n)
    - Space complexity: O(1)
*/
pub fn max_sub_array_optimized(nums: Vec<i32>) -> i32 {
    let mut prev_num = nums[0];
    let mut max_sum = nums[0];

    for i in 1..nums.len() {
        prev_num = nums[i] + if prev_num > 0 { prev_num } else { 0 };
        max_sum = max_sum.max(prev_num);
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array(vec![-3, 1, 3, -1, 2, -4, 2]), 5);
        assert_eq!(max_sub_array(vec![0, 1, -1, 0, 1, -1, 0, 1, -1]), 1);
        assert_eq!(max_sub_array(vec![-1, -2, -3, -4, -5]), -1);
        assert_eq!(max_sub_array(vec![-4, -3, -2, -1, 0]), 0);
        assert_eq!(max_sub_array(vec![-23, 30, -36, 42, -55, 62, -71, 88]), 88);
    }

    #[test]
    fn test_space_op_cases() {
        assert_eq!(max_sub_array_optimized(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array_optimized(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array_optimized(vec![-3, 1, 3, -1, 2, -4, 2]), 5);
        assert_eq!(max_sub_array_optimized(vec![0, 1, -1, 0, 1, -1, 0, 1, -1]), 1);
        assert_eq!(max_sub_array_optimized(vec![-1, -2, -3, -4, -5]), -1);
        assert_eq!(max_sub_array_optimized(vec![-4, -3, -2, -1, 0]), 0);
        assert_eq!(max_sub_array_optimized(vec![-23, 30, -36, 42, -55, 62, -71, 88]), 88);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![-1, 0]), 0);
        assert_eq!(max_sub_array(vec![0, 1]), 1);
        assert_eq!(max_sub_array(vec![i32::MIN, 0, i32::MAX]), i32::MAX);
    }
}

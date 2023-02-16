/*
    Given an array of positive integers nums and a positive integer target, return the
    minimal length of a subarray whose sum is greater than or equal to target. If there
    is no such subarray, return 0 instead.
*/

/*
   Sliding window method
   - Time complexity: O(n)
   - Space complexity: O(1)
*/
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut res = i32::MAX;
    let mut left = 0;
    let mut sum = 0;

    for right in 0..nums.len() {
        sum += nums[right];

        while sum >= target {
            res = res.min(right as i32 - left as i32 + 1);
            sum -= nums[left];
            left += 1;
        }
    }

    match res {
        i32::MAX => 0,
        _ => res,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(min_sub_array_len(1, vec![0]), 0);
        assert_eq!(min_sub_array_len(1, vec![1]), 1);
        assert_eq!(min_sub_array_len(1, vec![0, 1]), 1);
    }
}

/*
    Given an integer array nums sorted in non-decreasing order, return an array of
    the squares of each number sorted in non-decreasing order.
*/
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0 as i32;
    let mut right = nums.len() as i32 - 1;
    let mut res = vec![0; nums.len()];
    let mut curr = nums.len() as i32 - 1;

    while left <= right {
        let l = nums[left as usize] * nums[left as usize];
        let r = nums[right as usize] * nums[right as usize];

        if l > r {
            res[curr as usize] = l;
            left += 1;
        } else {
            res[curr as usize] = r;
            right -= 1;
        }

        curr -= 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(sorted_squares(vec![-4, -1, 0, 3, 10]), [0, 1, 9, 16, 100]);
        assert_eq!(sorted_squares(vec![-7, -3, 2, 3, 11]), [4, 9, 9, 49, 121]);
        assert_eq!(sorted_squares(vec![0, 2]), [0, 4]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(sorted_squares(vec![0]), [0]);
        assert_eq!(sorted_squares(vec![0, 1]), [0, 1]);
        assert_eq!(sorted_squares(vec![-1, 0, 1]), [0, 1, 1]);
    }
}

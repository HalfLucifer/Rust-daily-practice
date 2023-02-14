/*
    Given an integer array nums and an integer val, remove all occurrences of val in nums
    in-place. The relative order of the elements may be changed.

    If there are k elements after removing the duplicates, then the first k elements of
    nums should hold the final result. It does not matter what you leave beyond the first
    k elements. Return k after placing the final result in the first k slots of nums.
*/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut slow = 0;

    for fast in 0..nums.len() {
        if val != nums[fast] {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    slow as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let res = 2;
        assert_eq!(remove_element(&mut nums, val), res);
        assert_eq!(nums[..res as usize], [2, 2]);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let res = 5;
        assert_eq!(remove_element(&mut nums, val), res);
        assert_eq!(nums[..res as usize], [0, 1, 3, 0, 4]);
    }

    #[test]
    fn test_edge_cases() {
        let mut nums = vec![0];
        let val = 0;
        let res = 0;
        assert_eq!(remove_element(&mut nums, val), res);
        assert_eq!(nums[..res as usize], []);

        let mut nums = vec![0, 1];
        let val = 0;
        let res = 1;
        assert_eq!(remove_element(&mut nums, val), res);
        assert_eq!(nums[..res as usize], [1]);
    }
}

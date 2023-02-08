/*
    Suppose an array of length n sorted in ascending order is rotated between 1 and n times.

    Given the sorted rotated array nums of unique elements, return the minimum element of
    this array.
*/
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    // Converge left & right to find the min
    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] > nums[right] {
        // The pivot is in [mid+1..]
        // There is at least one value smaller than nums[mid], discard nums[mid]
            left = mid + 1;
        } else {
        // The pivot is in [..mid]
        // Because nums[mid] <= nums[right], don't discard nums[mid]
            right = mid;
        }
    }

    nums[left]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_min(vec![0]), 0);
    }
}

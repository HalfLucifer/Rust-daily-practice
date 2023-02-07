/*
    There is an integer array nums sorted in ascending order (with distinct values). Prior to
    being passed to your function, nums is possibly rotated at an unknown pivot index k.

    Given the array nums after the possible rotation and an integer target, return the index
    of target if it is in nums, or -1 if it is not in nums.
*/
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        // Found the target
        if nums[mid] == target {
            return mid as i32;
        }

        if nums[left] <= nums[mid] {
            // [left..=mid] is sorted, go binary searching
            if nums[left] <= target && target <= nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // [mid..=right] must be sorted, go binary searching
            if target >= nums[mid] && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(search(vec![0], 0), 0);
        assert_eq!(search(vec![1], 0), -1);
    }
}

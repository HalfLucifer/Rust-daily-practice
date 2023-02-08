/*
    Suppose an array of length n sorted in ascending order is rotated between 1 and n times.

    Given the sorted rotated array nums of unique elements, return the minimum element of
    this array.
*/
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if nums[mid] == nums[lo] {
            break;
        }

        if nums[mid] < nums[hi] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    nums[lo].min(nums[hi])
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

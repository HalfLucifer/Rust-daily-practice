/*
    Given an array nums containing n distinct numbers in the range [0..n],
    return the only number in the range that is missing from the array
*/

// Bit manipulation with XOR
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;

    for i in 0..=nums.len() {
        res ^= i as i32;
    }

    for i in 0..nums.len() {
        res ^= nums[i];
    }

    res
}

// Cyclic sort
pub fn missing_number_by_cyclic_sort(mut nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let n = nums.len();

    while i < n {
        let curr = nums[i] as usize;

        if curr < n && curr as i32 != nums[curr] {
            nums.swap(curr, i);
        } else {
            i += 1;
        }
    }

    for i in 0..nums.len() {
        if nums[i] != i as i32 {
            return i as i32;
        }
    }

    n as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(missing_number(vec![2, 0]), 1);
        assert_eq!(missing_number(vec![2, 1, 0]), 3);
        assert_eq!(missing_number(vec![4, 6, 2, 3, 5, 9, 0, 1, 7]), 8);

        assert_eq!(missing_number_by_cyclic_sort(vec![2, 0]), 1);
        assert_eq!(missing_number_by_cyclic_sort(vec![2, 1, 0]), 3);
        assert_eq!(missing_number_by_cyclic_sort(vec![4, 6, 2, 3, 5, 9, 0, 1, 7]), 8);

        let mut nums = (0..100).collect::<Vec<i32>>();
        nums.remove(50);
        assert_eq!(missing_number(nums.clone()), 50);
        assert_eq!(missing_number_by_cyclic_sort(nums), 50);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(missing_number(vec![]), 0);
        assert_eq!(missing_number(vec![0]), 1);

        assert_eq!(missing_number_by_cyclic_sort(vec![]), 0);
        assert_eq!(missing_number_by_cyclic_sort(vec![0]), 1);
    }
}

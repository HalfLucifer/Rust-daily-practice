/*
    Given a non-empty array of integers nums, every element appears twice
    except for one. Find that single one.
*/
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;

    for i in 0..nums.len() {
        res ^= nums[i];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(single_number(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8]), 8);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(single_number(vec![]), 0);
        assert_eq!(single_number(vec![0]), 0);
        assert_eq!(single_number(vec![1]), 1);
    }
}

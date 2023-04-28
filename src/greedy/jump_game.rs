/*
    Given an integer array nums. You are initially positioned at the array's
    first index, and each element in the array represents your maximum jump
    length at that position.

    Return true if you can reach the last index, or false otherwise.
*/
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut distance = 0;
    let mut i = 0;

    while i <= distance {
        // Greedy: farthest distance it can reach
        distance = distance.max(i + nums[i] as usize);

        if distance >= nums.len() - 1 {
            return true;
        }

        i += 1;
    }

    false
}

pub fn can_jump_method2(nums: Vec<i32>) -> bool {
    let mut res = 0;

    for i in 0..nums.len() {
        if i > res {
            break;
        }

        res = res.max(nums[i] as usize + i);
    }

    res >= nums.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump_method2(vec![2, 3, 1, 1, 4]), true);

        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(can_jump_method2(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(can_jump(vec![0]), true);
        assert_eq!(can_jump_method2(vec![0]), true);

        assert_eq!(can_jump(vec![1, 0]), true);
        assert_eq!(can_jump_method2(vec![1, 0]), true);
    }
}

/*
    Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is
    nums[0]), return the next greater number for every element in nums.

    The next greater number of a number x is the first greater number to its traversing-order
    next in the array, which means you could search circularly to find its next greater number.
    If it doesn't exist, return -1 for this number.
*/
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut result = vec![-1; len];
    let mut stack = vec![];

    for i in (0..len * 2 - 1).rev() {
        let num = nums[i % len];

        while let Some(&n) = stack.last() {
            if n <= num {
                stack.pop();
            } else {
                break;
            }
        }

        if !stack.is_empty() {
            result[i % len] = *stack.last().unwrap();
        }

        stack.push(num);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
        assert_eq!(next_greater_elements(vec![1, 2, 3, 4, 3]), vec![2, 3, 4, -1, 4]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(next_greater_elements(vec![0]), vec![-1]);
    }
}

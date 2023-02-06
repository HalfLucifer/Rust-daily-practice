/*
    Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order,
    find two numbers such that they add up to a specific target number.

    Return the indices of the two numbers, index1 and index2, added by one as an integer array
    [index1, index2] of length 2.
*/
use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let sum = numbers[left] + numbers[right];

        match sum.cmp(&target) {
            Ordering::Greater => {
                right -= 1;
            }

            Ordering::Less => {
                left += 1;
            }

            Ordering::Equal => {
                break;
            }
        }
    }

    vec![left as i32 + 1, right as i32 + 1]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [1, 2]);
        assert_eq!(two_sum(vec![2, 3, 4], 6), [1, 3]);
        assert_eq!(two_sum(vec![-1, 0], -1), [1, 2]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(two_sum(vec![0, 0], 0), [1, 2]);
        assert_eq!(two_sum(vec![i32::MIN, i32::MIN + 1, i32::MAX], 0), [2, 3]);
    }
}

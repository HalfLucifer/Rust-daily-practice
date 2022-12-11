/*
   Find all non-repeating pairs summed to the target number
*/
pub fn find_all_two_sum(arr: &[i32], target: i32) -> Vec<(i32, i32)> {
    if arr.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut input = arr.to_vec();
    let mut left = 0;
    let mut right = input.len() - 1;

    // Sort in ascending order
    input.sort();

    while left < right {
        let sum = input[left] + input[right];

        if sum > target {
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            let l = input[left];
            let r = input[right];

            result.push((l, r));

            // Skip repeated numbers
            while input[left] == l {
                left += 1;
            }

            while input[right] == r {
                right -= 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(find_all_two_sum(&arr, 2), []);
        assert_eq!(find_all_two_sum(&arr, 3), [(1, 2)]);
        assert_eq!(find_all_two_sum(&arr, 4), [(1, 3)]);
        assert_eq!(find_all_two_sum(&arr, 5), [(1, 4), (2, 3)]);

        let arr = [1, 2, 3, 4, 0, 3, 2, 1, 0, 4, 5, -1, -2, 6];
        assert_eq!(find_all_two_sum(&arr, 2), [(-2, 4), (-1, 3), (0, 2), (1, 1)]);
        assert_eq!(find_all_two_sum(&arr, 3), [(-2, 5), (-1, 4), (0, 3), (1, 2)]);
        assert_eq!(find_all_two_sum(&arr, 4), [(-2, 6), (-1, 5), (0, 4), (1, 3), (2, 2)]);
        assert_eq!(find_all_two_sum(&arr, 5), [(-1, 6), (0, 5), (1, 4), (2, 3)]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_all_two_sum(&[], 0), []);
        assert_eq!(find_all_two_sum(&[0], 0), []);
        assert_eq!(find_all_two_sum(&[0, 1], 1), [(0, 1)]);
        assert_eq!(find_all_two_sum(&[-1, 0, 1], 0), [(-1, 1)]);
        assert_eq!(find_all_two_sum(&[-1, 0, 0, 1], 0), [(-1, 1), (0, 0)]);

        assert_eq!(
            find_all_two_sum(&[i32::MIN, -1, 0, i32::MAX], -1),
            [(i32::MIN, i32::MAX), (-1, 0)]
        );
    }
}

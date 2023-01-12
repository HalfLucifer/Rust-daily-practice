/*
    Find all valid combinations of k numbers that sum up to n such that the following
    conditions are true:
    - Only numbers 1 through 9 are used.
    - Each number is used at most once.
*/
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn backtrack(result: &mut Vec<Vec<i32>>, track: &mut Vec<i32>, k: usize, target: i32, start: i32) {
        if track.len() == k {
            if target == 0 {
                result.push(track.to_vec());
            }
            return;
        }

        for i in start..=9 {
            track.push(i);
            backtrack(result, track, k, target - i, i + 1);
            track.pop();
        }
    }

    let mut result = vec![];
    let mut track = vec![];
    backtrack(&mut result, &mut track, k as usize, n, 1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(combination_sum3(3, 7), [vec![1, 2, 4]]);
        assert_eq!(combination_sum3(3, 9), [vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]);

        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(combination_sum3(4, 1), expected);
    }

    #[test]
    fn test_edge_cases() {
        for i in 1..=9 {
            assert_eq!(combination_sum3(1, i), [vec![i]]);
        }
    }
}

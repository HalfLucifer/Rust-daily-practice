/*
    Given a collection of candidate numbers and a target number, find all unique
    combinations in candidates where the candidate numbers sum to target.

    Each number in candidates may only be used once in the combination.
*/
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(candidates: &Vec<i32>, track: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, start: usize, target: i32) {
        if target == 0 {
            result.push(track.to_vec());
            return;
        }

        for i in start..candidates.len() {
            // Prune the branches 1)
            if target < candidates[i] {
                break;
            }

            // Skip the element already used
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }

            track.push(candidates[i]);
            backtrack(candidates, track, result, i + 1, target - candidates[i]);
            track.pop();
        }
    }

    let mut track: Vec<i32> = vec![];
    let mut result: Vec<Vec<i32>> = vec![];

    candidates.sort_unstable(); // Sort to align the same numbers
    backtrack(&candidates, &mut track, &mut result, 0, target);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_once_cases() {
        assert_eq!(combination_sum2(vec![1, 2, 3], 6), [vec![1, 2, 3]]);
        assert_eq!(combination_sum2(vec![2, 3, 6, 7], 7), [vec![7]]);
        assert_eq!(combination_sum2(vec![2, 3, 5], 8), [vec![3, 5]]);
        assert_eq!(combination_sum2(vec![3, 5, 8], 11), [vec![3, 8]]);

        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(combination_sum2(vec![7, 3, 2], 18), expected);

        assert_eq!(
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            [vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );

        assert_eq!(
            combination_sum2(vec![2, 5, 2, 1, 2], 5),
            [vec![1, 2, 2], vec![5]]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(combination_sum2(vec![], 0), [vec![]]);
        assert_eq!(combination_sum2(vec![1], 1), [vec![1]]);
    }
}

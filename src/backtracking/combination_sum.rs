/*
    Given an array of distinct integers candidates and a target integer target, return a
    list of all unique combinations of candidates where the chosen numbers sum to target.

    The same number may be chosen from candidates an unlimited number of times.
*/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(candidates: &Vec<i32>, track: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, start: usize, target: i32) {
        if target <= 0 {
            if target == 0 {
                result.push(track.to_vec());                
            }
            return;
        }
        
        for i in start..candidates.len() {
            track.push(candidates[i]);
            backtrack(candidates, track, result, i, target - candidates[i]);
            track.pop();
        }
    }

    let mut track: Vec<i32> = vec![];
    let mut result: Vec<Vec<i32>> = vec![];
    backtrack(&candidates, &mut track, &mut result, 0, target);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(combination_sum(vec![2, 3, 6, 7], 7), [vec![2, 2, 3], vec![7]]);
        assert_eq!(combination_sum(vec![2, 3, 5], 8), [vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
        assert_eq!(combination_sum(vec![3, 5, 8], 11), [vec![3, 3, 5], vec![3, 8]]);

        assert_eq!(
            combination_sum(vec![1, 2, 3], 6),
            [
                vec![1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 2],
                vec![1, 1, 1, 3],
                vec![1, 1, 2, 2],
                vec![1, 2, 3],
                vec![2, 2, 2],
                vec![3, 3]
            ]
        );

        assert_eq!(
            combination_sum(vec![7, 3, 2], 18),
            [
                vec![7, 7, 2, 2],
                vec![7, 3, 3, 3, 2],
                vec![7, 3, 2, 2, 2, 2],
                vec![3, 3, 3, 3, 3, 3],
                vec![3, 3, 3, 3, 2, 2, 2],
                vec![3, 3, 2, 2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2, 2, 2, 2, 2]
            ]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(combination_sum(vec![], 0), [vec![]]);
        assert_eq!(combination_sum(vec![1], 1), [vec![1]]);
    }
}

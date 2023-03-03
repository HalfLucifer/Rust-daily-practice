/*
    Given an integer array nums that may contain duplicates, return all possible subsets.
    The solution set must not contain duplicate subsets.
*/
pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &Vec<i32>, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: usize) {
        res.push(track.clone());

        for i in start..nums.len() {
            if i > start && nums[i - 1] == nums[i] {
                continue;
            }

            track.push(nums[i]);
            backtrack(nums, track, res, i + 1);
            track.pop();
        }
    }

    let mut res = vec![];
    let mut track = vec![];

    nums.sort_unstable(); // Sort to skip duplicates later
    backtrack(&nums, &mut track, &mut res, 0);
    res
}

pub fn subsets_with_dup_by_set(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    fn backtrack(nums: &Vec<i32>, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: usize) {
        res.push(track.clone());

        let mut visited = HashSet::new();

        for i in start..nums.len() {
            if visited.contains(&nums[i]) {
                continue;
            }

            visited.insert(nums[i]);
            
            track.push(nums[i]);
            backtrack(nums, track, res, i + 1);
            track.pop();
        }
    }

    let mut res = vec![];
    let mut track = vec![];

    nums.sort_unstable(); // Sort to skip duplicates later
    backtrack(&nums, &mut track, &mut res, 0);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            subsets_with_dup(
                vec![1, 2, 2]),
                vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]]
        );

        assert_eq!(
            subsets_with_dup_by_set(
                vec![1, 2, 2]),
                vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
        assert_eq!(subsets_with_dup_by_set(vec![0]), vec![vec![], vec![0]]);
    }
}

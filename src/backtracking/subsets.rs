/*
    Given an integer array nums of unique elements, return all possible subsets.
    The solution set must not contain duplicate subsets. 
 */
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &Vec<i32>, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: usize) {
        res.push(track.clone());

        for i in start..nums.len() {
            track.push(nums[i]);
            backtrack(nums, track, res, i + 1);
            track.pop();
        }
    }

    let mut res = vec![];
    let mut track = vec![];
    backtrack(&nums, &mut track, &mut res, 0);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut res = subsets(vec![1, 2, 3]);
        res.sort();
        assert_eq!(res, vec![vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3], vec![2], vec![2, 3], vec![3]]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(subsets(vec![0]), vec![vec![], vec![0]]);
    }
}

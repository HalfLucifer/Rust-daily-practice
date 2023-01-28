/*
    Given an array nums of distinct integers, return all the possible permutations.
*/
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, track: &mut Vec<i32>, visited: &mut Vec<bool>) {
        if track.len() == nums.len() {
            res.push(track.clone());
            return;
        }

        for i in 0..nums.len() {
            if !visited[i] {
                // Select the element
                visited[i] = true;
                track.push(nums[i]);

                backtrack(nums, res, track, visited);

                // Unselect the element
                track.pop();
                visited[i] = false;
            }
        }
    }

    let mut res = vec![];
    let mut track = vec![];
    let mut visited = vec![false; nums.len()];
    backtrack(&nums, &mut res, &mut track, &mut visited);
    res
}

pub fn permute_by_swap(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: usize) {
        if start == nums.len() {
            res.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i);
            backtrack(nums, res, start + 1);
            nums.swap(i, start);
        }
    }

    let mut res = vec![];
    backtrack(&mut nums, &mut res, 0);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut res = permute(vec![1, 2, 3]);
        res.sort_unstable();
        assert_eq!(
            res,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );

        let mut res = permute_by_swap(vec![1, 2, 3]);
        res.sort_unstable();
        assert_eq!(
            res,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(permute(vec![0]), vec![vec![0]]);
        assert_eq!(permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);

        assert_eq!(permute_by_swap(vec![0]), vec![vec![0]]);
        assert_eq!(permute_by_swap(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
    }
}

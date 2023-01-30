/*
    Given a collection of numbers that might contain duplicates, return all
    possible unique permutations.
*/
pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, track: &mut Vec<i32>, visited: &mut Vec<bool>) {
        if track.len() == nums.len() {
            res.push(track.clone());
            return;
        }

        for i in 0..nums.len() {
            // Skip visited element
            if visited[i] {
                continue;
            }

            // When a number is the same with previous one, skip it if the previous
            // one was NOT already visited
            // NOTE: && visited[i - 1] can also work but it is less efficient
            if i > 0 && nums[i - 1] == nums[i] && !visited[i - 1] {
                continue;
            }

            visited[i] = true;
            track.push(nums[i]);

            backtrack(nums, res, track, visited);

            track.pop();
            visited[i] = false;
        }
    }

    let mut res = vec![];
    let mut track = vec![];
    let mut visited = vec![false; nums.len()];

    nums.sort_unstable(); // Sort to skip duplicates later
    backtrack(&nums, &mut res, &mut track, &mut visited);
    res
}

pub fn permute_unique_by_hashset(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    fn backtrack(nums: &Vec<i32>, res: &mut HashSet<Vec<i32>>, track: &mut Vec<i32>, visited: &mut HashSet<usize>) {
        if track.len() == nums.len() {
            // Use HashSet to filter duplicates out
            if !res.contains(track) {
                res.insert(track.clone());
            }
            return;
        }

        for i in 0..nums.len() {
            if !visited.contains(&i) {
                visited.insert(i);
                track.push(nums[i]);

                backtrack(nums, res, track, visited);

                track.pop();
                visited.remove(&i);
            }
        }
    }

    let mut res = HashSet::new();
    let mut track = vec![];
    let mut visited = HashSet::new();

    backtrack(&nums, &mut res, &mut track, &mut visited);
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut res = permute_unique(vec![1, 1, 2]);
        res.sort_unstable();
        assert_eq!(res, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);

        let mut res = permute_unique_by_hashset(vec![1, 1, 2]);
        res.sort_unstable();
        assert_eq!(res, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);

        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        let mut res = permute_unique(vec![1, 2, 3]);
        res.sort_unstable();
        assert_eq!(res, expected);

        let mut res = permute_unique_by_hashset(vec![1, 2, 3]);
        res.sort_unstable();
        assert_eq!(res, expected);

        let expected = vec![
            vec![1, 2, 3, 3],
            vec![1, 3, 2, 3],
            vec![1, 3, 3, 2],
            vec![2, 1, 3, 3],
            vec![2, 3, 1, 3],
            vec![2, 3, 3, 1],
            vec![3, 1, 2, 3],
            vec![3, 1, 3, 2],
            vec![3, 2, 1, 3],
            vec![3, 2, 3, 1],
            vec![3, 3, 1, 2],
            vec![3, 3, 2, 1],
        ];

        let mut res = permute_unique(vec![1, 2, 3, 3]);
        res.sort_unstable();
        assert_eq!(res, expected);

        let mut res = permute_unique_by_hashset(vec![1, 2, 3, 3]);
        res.sort_unstable();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_edge_cases() {
        let res = permute_unique(vec![0]);
        assert_eq!(res, vec![vec![0]]);

        let res = permute_unique_by_hashset(vec![0]);
        assert_eq!(res, vec![vec![0]]);

        let res = permute_unique(vec![0, 0]);
        assert_eq!(res, vec![vec![0, 0]]);

        let res = permute_unique_by_hashset(vec![0, 0]);
        assert_eq!(res, vec![vec![0, 0]]);
    }
}

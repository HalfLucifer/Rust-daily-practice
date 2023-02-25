/*
    Given two integers n and k, return all possible combinations of k numbers chosen
    from the range [1, n].
*/
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn backtrack(n: i32, k: i32, start: i32, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if k == 0 {
            res.push(track.clone());
            return;
        }

        // NOTE: n-k+1 is an optimized range, while n can also work
        // e.g. for C(4,2) case, no need to proceed element started from 4
        let end = n - k + 1;

        for i in start..=end {
            track.push(i);
            backtrack(n, k - 1, i + 1, track, res);
            track.pop();
        }
    }

    let mut res = vec![];
    let mut track = vec![];
    backtrack(n, k, 1, &mut track, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(combine(3, 1), vec![vec![1], vec![2], vec![3]]);
        assert_eq!(combine(4, 2), vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]);
        assert_eq!(combine(5, 3), vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 2, 5], vec![1, 3, 4], vec![1, 3, 5], vec![1, 4, 5], vec![2, 3, 4], vec![2, 3, 5], vec![2, 4, 5], vec![3, 4, 5]]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(combine(1, 1), vec![vec![1]]);
    }
}

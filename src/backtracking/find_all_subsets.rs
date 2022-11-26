pub fn find_all_subsets(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut track = vec![];
    backtrack(arr, &mut result, &mut track, 0);
    result
}

fn backtrack(arr: &[i32], result: &mut Vec<Vec<i32>>, track: &mut Vec<i32>, start: i32) {
    result.push(track.clone());

    for i in start..arr.len() as i32 {
        track.push(arr[i as usize]);
        backtrack(arr, result, track, i + 1);
        track.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2num() {
        let a = [i32::MIN, i32::MAX];
        let ans = vec![vec![], vec![i32::MIN], vec![i32::MIN, i32::MAX], vec![i32::MAX]];
        let result = find_all_subsets(&a);
        let matched_count = result.iter().zip(ans.iter()).filter(|&(x, y)|x == y).count();
        assert!(matched_count == result.len() && matched_count == ans.len());
    }

    #[test]
    fn test_3num() {
        let a = [1, 2, 3];
        let ans = vec![vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3], vec![2], vec![2, 3], vec![3]];
        let result = find_all_subsets(&a);
        let matched_count = result.iter().zip(ans.iter()).filter(|&(x, y)|x == y).count();
        assert!(matched_count == result.len() && matched_count == ans.len());
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_all_subsets(&[]), vec![vec![]]);
        assert_eq!(find_all_subsets(&[0]), vec![vec![], vec![0]]);
    }
}
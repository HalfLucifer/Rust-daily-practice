use std::collections::HashMap;

pub fn two_sum(arr: &[i32], sum: i32) -> (Option<usize>, Option<usize>) {
    let mut hm = HashMap::new();

    for i in 0..arr.len() {
        let target = sum - arr[i];

        if let Some(j) = hm.get(&target) {
            return (Some(*j), Some(i));
        }

        hm.insert(arr[i], i);
    }

    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 3), (Some(0), Some(1)));
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 4), (Some(0), Some(2)));
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 5), (Some(1), Some(2)));
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 6), (Some(1), Some(3)));
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 7), (Some(2), Some(3)));
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 8), (Some(2), Some(4)));
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 9), (Some(3), Some(4)));
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 10), (None, None));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(two_sum(&[], 0), (None, None));
        assert_eq!(two_sum(&[0], 0), (None, None));
        assert_eq!(two_sum(&[0, 1], 1), (Some(0), Some(1)));
        assert_eq!(two_sum(&[-1, 0, 1], 0), (Some(0), Some(2)));
    }
}

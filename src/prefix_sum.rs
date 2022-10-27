pub fn prefix_sum(arr: &[i32], range: (usize, usize)) -> i32 {
    assert!(!arr.is_empty());
    assert!(range.0 <= range.1);

    if range.0 > arr.len() || range.1 > arr.len() {
        return 0;
    }

    let mut prefix_sum = vec![0; arr.len() + 1];

    for i in 0..arr.len() {
        prefix_sum[i + 1] = prefix_sum[i] + arr[i];
    }

    if range.0 == 0 {
        prefix_sum[range.1 + 1]
    } else {
        prefix_sum[range.1 + 1] - prefix_sum[range.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let ranges = [(0, 1), (1, 3), (3, 5), (5, 7), (7, 9), (0, 9), (5, 5)];

        ranges.iter().for_each(|range| {
            assert_eq!(
                prefix_sum(&arr, *range),
                (range.0..=range.1).map(|v| arr[v]).sum::<i32>()
            );
        });
    }

    #[test]
    fn test_edge_cases() {
        let arr = [1];
        assert_eq!(prefix_sum(&arr, (1, 2)), 0);
        assert_eq!(prefix_sum(&arr, (0, 0)), 1);

        let arr = [i32::MIN, 0, i32::MAX];
        assert_eq!(prefix_sum(&arr, (0, 1)), i32::MIN);
        assert_eq!(prefix_sum(&arr, (0, 2)), -1);
    }
}

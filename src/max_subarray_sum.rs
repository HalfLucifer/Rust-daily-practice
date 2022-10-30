pub fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut prev = arr[0];
    let mut curr = arr[0];
    let mut result = arr[0];

    for i in 1..arr.len() {
        curr = arr[i].max(arr[i] + prev);
        result = result.max(curr);
        prev = curr;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(max_subarray_sum(&[-3, 1, 3, -1, 2, -4, 2]), 5);
        assert_eq!(max_subarray_sum(&[0, 1, -1, 0, 1, -1, 0, 1, -1]), 1);
        assert_eq!(max_subarray_sum(&[-1, -2, -3, -4, -5]), -1);
        assert_eq!(max_subarray_sum(&[-4, -3, -2, -1, 0]), 0);
        assert_eq!(max_subarray_sum(&[-23, 30, -36, 42, -55, 62, -71, 88]), 88);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(max_subarray_sum(&[]), 0);
        assert_eq!(max_subarray_sum(&[0]), 0);
        assert_eq!(max_subarray_sum(&[-1, 0]), 0);
        assert_eq!(max_subarray_sum(&[0, 1]), 1);
        assert_eq!(max_subarray_sum(&[i32::MIN, 0, i32::MAX]), i32::MAX);
    }
}

/*
  Build a difference array to update value in range
  - Update value: O(1)
  - Output array: O(n)
*/
pub fn range_update(arr: &mut [i32], range: (usize, usize), value: i32) {
    assert!(!arr.is_empty());
    assert!(range.0 <= range.1);
    assert!(range.1 < arr.len());

    // Build a difference array
    let mut difference_array = vec![0; arr.len()];
    difference_array[0] = arr[0];

    for i in 1..arr.len() {
        difference_array[i] = arr[i] - arr[i - 1];
    }

    // Update value by difference array
    difference_array[range.0] += value;

    if range.1 + 1 < arr.len() {
        difference_array[range.1 + 1] -= value;
    }

    // Rebuild the output array
    arr[0] = difference_array[0];

    for i in 1..arr.len() {
        arr[i] = arr[i - 1] + difference_array[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut arr = (1..=10).collect::<Vec<_>>();

        range_update(&mut arr, (0, 1), 10);
        assert_eq!(arr, [11, 12, 3, 4, 5, 6, 7, 8, 9, 10]);

        range_update(&mut arr, (2, 8), 30);
        assert_eq!(arr, [11, 12, 33, 34, 35, 36, 37, 38, 39, 10]);

        range_update(&mut arr, (7, 9), -20);
        assert_eq!(arr, [11, 12, 33, 34, 35, 36, 37, 18, 19, -10]);

        range_update(&mut arr, (0, 9), 100);
        assert_eq!(arr, [111, 112, 133, 134, 135, 136, 137, 118, 119, 90]);

        range_update(&mut arr, (5, 5), -136);
        assert_eq!(arr, [111, 112, 133, 134, 135, 0, 137, 118, 119, 90]);
    }

    #[test]
    fn test_edge_cases() {
        let mut arr = [0];
        range_update(&mut arr, (0, 0), 1);
        assert_eq!(arr, [1]);

        let mut arr = [-1, 0, 1];
        range_update(&mut arr, (0, 1), 1);
        range_update(&mut arr, (1, 2), -1);
        assert_eq!(arr, [0, 0, 0]);
    }
}

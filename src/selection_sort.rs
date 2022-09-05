pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut target = i;

        for j in i + 1..arr.len() {
            if arr[target] > arr[j] {
                target = j;
            }
        }

        arr.swap(i, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut a = [0];
        selection_sort(&mut a);
        assert_eq!(a, [0]);
    }

    #[test]
    fn test_sorted() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        selection_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_reversed() {
        let mut a = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        selection_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_unordered() {
        let mut a = [2, 5, 3, 10, 8, 9, 1, 4, 6, 7];
        selection_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_zeros_and_ones() {
        let mut a = [1, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        selection_sort(&mut a);
        assert_eq!(a, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
    }
}

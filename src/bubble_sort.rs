/*
   time complexity O(n^2)
   space complexity O(1)
*/
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut is_sorted = false;

    while !is_sorted {
        is_sorted = true;

        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                is_sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut a = [0];
        bubble_sort(&mut a);
        assert_eq!(a, [0]);
    }

    #[test]
    fn test_sorted() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        bubble_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_reversed() {
        let mut a = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        bubble_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_unordered() {
        let mut a = [2, 5, 3, 10, 8, 9, 1, 4, 6, 7];
        bubble_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_zeros_and_ones() {
        let mut a = [1, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        bubble_sort(&mut a);
        assert_eq!(a, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
    }
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut target = i;

        while target > 0 {
            match arr[target - 1] > arr[target] {
                true => {
                    arr.swap(target, target - 1);
                    target -= 1;
                }
                false => break,
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
        insertion_sort(&mut a);
        assert_eq!(a, [0]);
    }

    #[test]
    fn test_sorted() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        insertion_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_reversed() {
        let mut a = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        insertion_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_unordered() {
        let mut a = [2, 5, 3, 10, 8, 9, 1, 4, 6, 7];
        insertion_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_zeros_and_ones() {
        let mut a = [1, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        insertion_sort(&mut a);
        assert_eq!(a, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
    }
}

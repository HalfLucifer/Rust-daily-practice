pub fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    fn quick_sort_imp<T: Ord + Copy>(arr: &mut [T], lo: isize, hi: isize) {
        if lo > hi {
            return;
        }
    
        let pivot = partition(arr, lo, hi);
    
        quick_sort_imp(arr, lo, pivot - 1);
        quick_sort_imp(arr, pivot + 1, hi);
    }

    let hi = arr.len() as isize - 1;
    quick_sort_imp(arr, 0, hi);
}

fn partition<T: Ord + Copy>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = arr[hi as usize];
    let mut index = lo;

    for i in lo..hi {
        if arr[i as usize] < pivot {
            arr.swap(i as usize, index as usize);
            index += 1;
        }
    }

    arr.swap(hi as usize, index as usize);

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut a = [0];
        quick_sort(&mut a);
        assert_eq!(a, [0]);
    }

    #[test]
    fn test_normal() {
        let mut a = [20, 5, 10, 7, 11, 14, 2, 8, 1, 13];
        quick_sort(&mut a);
        assert_eq!(a, [1, 2, 5, 7, 8, 10, 11, 13, 14, 20]);
    }

    #[test]
    fn test_zeros_and_ones() {
        let mut a = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        quick_sort(&mut a);
        assert_eq!(a, [0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_accending() {
        let mut a = [9, 99, 999, 9999, 99999, 999999, 9999999];
        quick_sort(&mut a);
        assert_eq!(a, [9, 99, 999, 9999, 99999, 999999, 9999999]);
    }

    #[test]
    fn test_descending() {
        let mut a = [9999999, 999999, 99999, 9999, 999, 99, 9];
        quick_sort(&mut a);
        assert_eq!(a, [9, 99, 999, 9999, 99999, 999999, 9999999]);
    }
}

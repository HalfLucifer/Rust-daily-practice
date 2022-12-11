use std::cmp::Ordering;

pub fn find_kth_largest_number(arr: &mut [i32], k: usize) -> Option<i32> {
    if k < 1 || k > arr.len() {
        return None;
    }

    Some(quick_select(arr, arr.len() - k))
}

/*
 - Time complexity
   - best & average case: O(n)
   - worst case: O(n^2)
 - Space complexity: O(1)
*/
fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    fn quick_select_recursive(arr: &mut [i32], lo: isize, hi: isize, k: usize) -> i32 {
        let pivot = partition(arr, lo, hi) as usize;

        match pivot.cmp(&k) {
            Ordering::Greater => quick_select_recursive(arr, lo, pivot as isize - 1, k),
            Ordering::Less => quick_select_recursive(arr, pivot as isize + 1, hi, k),
            Ordering::Equal => arr[pivot],
        }
    }

    quick_select_recursive(arr, 0, (arr.len() - 1) as isize, k)
}

fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    let pivot = arr[hi as usize];
    let mut pivot_index = lo;

    for i in lo..hi {
        if arr[i as usize] < pivot {
            arr.swap(i as usize, pivot_index as usize);
            pivot_index += 1;
        }
    }

    arr.swap(hi as usize, pivot_index as usize);
    pivot_index as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut arr = [2, 10, 7, 6, 1, 5, 9, 3, 8, 4];

        for i in 1..=10 {
            assert_eq!(find_kth_largest_number(&mut arr, i), Some(11 - i as i32));
        }

        assert_eq!(find_kth_largest_number(&mut arr, 11), None);
        assert_eq!(find_kth_largest_number(&mut arr, 0), None);
    }

    #[test]
    fn test_edge_cases() {
        let mut arr = [];
        assert_eq!(find_kth_largest_number(&mut arr, 1), None);

        let mut arr = [0];
        assert_eq!(find_kth_largest_number(&mut arr, 0), None);
        assert_eq!(find_kth_largest_number(&mut arr, 1), Some(0));

        let mut arr = [0, i32::MIN, i32::MAX];
        assert_eq!(find_kth_largest_number(&mut arr, 1), Some(i32::MAX));
        assert_eq!(find_kth_largest_number(&mut arr, 2), Some(0));
        assert_eq!(find_kth_largest_number(&mut arr, 3), Some(i32::MIN));
    }

    #[test]
    fn test_sorted_cases() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut arr_rev = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        for i in 1..=10 {
            assert_eq!(find_kth_largest_number(&mut arr, i), Some(11 - i as i32));
            assert_eq!(
                find_kth_largest_number(&mut arr_rev, i),
                Some(11 - i as i32)
            );
        }
    }
}

use std::cmp::Ordering;

pub fn binary_search_range<T: Ord>(arr: &[T], target: &T) -> Option<(usize, usize)> {
    let end = arr.len() as isize - 1;

    if let Some(pos) = binary_search(arr, 0, end, target) {
        let mut leftmost = pos;
        let mut rightmost = pos;
        let mut start_pos = pos;

        // Find the leftmost position of target value
        while let Some(l) = binary_search(arr, 0, start_pos as isize - 1, target) {
            leftmost = l;
            start_pos = l;
        }

        start_pos = pos;

        // Find the rightmost position of target value
        while let Some(r) = binary_search(arr, start_pos as isize + 1, end, target) {
            rightmost = r;
            start_pos = r;
        }

        return Some((leftmost, rightmost));
    }

    None
}

pub fn binary_search<T: Ord>(arr: &[T], start: isize, end: isize, target: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut start = start;
    let mut end = end;

    while end >= start {
        // Preventing from integer overflow
        let mid = start + (end - start) / 2;

        match target.cmp(&arr[mid as usize]) {
            Ordering::Greater => {
                start = mid + 1;
            }

            Ordering::Less => {
                end = mid - 1;
            }

            Ordering::Equal => {
                return Some(mid as usize);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search_range(&v, &5), Some((5, 5)));

        let v = vec![1, 2, 3, 4, 4, 4, 4, 4, 4, 5, 5, 6, 6, 7, 8, 9];
        assert_eq!(binary_search_range(&v, &4), Some((3, 8)));
    }

    #[test]
    fn test_edge_cases() {
        let v = [];
        assert_eq!(binary_search_range(&v, &0), None);
        
        assert_eq!(binary_search_range(&[0], &0), Some((0, 0)));
        assert_eq!(binary_search_range(&[0], &1), None);
        assert_eq!(binary_search_range(&[0, 0], &0), Some((0, 1)));
        assert_eq!(binary_search_range(&[0; 100], &0), Some((0, 99)));
    }

    #[test]
    fn test_leftmost_rightmost_cases() {
        let v = vec![0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search_range(&v, &0), Some((0, 4)));

        let v = vec![-9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 0, 0, 0, 0];
        assert_eq!(binary_search_range(&v, &0), Some((9, 13)));

        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search_range(&v, &0), Some((0, 0)));
        assert_eq!(binary_search_range(&v, &9), Some((9, 9)));
    }
}

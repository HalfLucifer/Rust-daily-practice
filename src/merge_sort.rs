pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut ret);

    arr.copy_from_slice(&ret);
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], ret: &mut [T]) {
    let mut lp = 0;
    let mut rp = 0;
    let mut index = 0;

    while lp < left.len() && rp < right.len() {
        if left[lp] < right[rp] {
            ret[index] = left[lp];
            lp += 1;
            index += 1;
        } else {
            ret[index] = right[rp];
            rp += 1;
            index += 1;
        }
    }

    if lp < rp {
        ret[index..].copy_from_slice(&left[lp..]);
    } else if lp < rp {
        ret[index..].copy_from_slice(&right[rp..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut a = [0];
        merge_sort(&mut a);
        assert_eq!(a, [0]);
    }

    #[test]
    fn test_normal() {
        let mut a = [20, 5, 10, 7, 11, 14, 2, 8, 1, 13];
        merge_sort(&mut a);
        assert_eq!(a, [1, 2, 5, 7, 8, 10, 11, 13, 14, 20]);
    }

    #[test]
    fn test_zeros_and_ones() {
        let mut a = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        merge_sort(&mut a);
        assert_eq!(a, [0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_accending() {
        let mut a = [9, 99, 999, 9999, 99999, 999999, 9999999];
        merge_sort(&mut a);
        assert_eq!(a, [9, 99, 999, 9999, 99999, 999999, 9999999]);
    }

    #[test]
    fn test_descending() {
        let mut a = [9999999, 999999, 99999, 9999, 999, 99, 9];
        merge_sort(&mut a);
        assert_eq!(a, [9, 99, 999, 9999, 99999, 999999, 9999999]);
    }
}

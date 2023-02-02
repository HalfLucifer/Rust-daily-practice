pub fn reverse_array(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        arr.swap(left, right);
        
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut a = (1..100).collect::<Vec<_>>();
        let b = a.iter().copied().rev().collect::<Vec<_>>();
        reverse_array(&mut a);
        assert_eq!(a, b);

        a.pop();
        let b = a.iter().copied().rev().collect::<Vec<_>>();
        reverse_array(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn test_rev_rev_cases() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        reverse_array(&mut a);
        reverse_array(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut a = [0, 0, 0];
        reverse_array(&mut a);
        reverse_array(&mut a);
        assert_eq!(a, [0, 0, 0]);

        let mut a = [1];
        reverse_array(&mut a);
        reverse_array(&mut a);
        assert_eq!(a, [1]);
    }

    #[test]
    fn test_edge_cases() {
        let mut a = [];
        reverse_array(&mut a);
        assert_eq!(a, []);

        let mut a = [0];
        reverse_array(&mut a);
        assert_eq!(a, [0]);

        let mut a = [i32::MIN, 0, i32::MAX];
        reverse_array(&mut a);
        assert_eq!(a, [i32::MAX, 0, i32::MIN]);
    }
}

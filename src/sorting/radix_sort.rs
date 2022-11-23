pub fn radix_sort(arr: &mut [u32]) {
    let max_len = arr.iter().max().unwrap_or(&0).to_string().len();

    for i in 0..max_len {
        arr.sort_by(|a, b| {
            let v1 = a.to_string().chars().rev().nth(i).unwrap_or('0');
            let v2 = b.to_string().chars().rev().nth(i).unwrap_or('0');
            v1.cmp(&v2)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut arr = (0..=10).collect::<Vec<_>>();
        radix_sort(&mut arr);
        assert_eq!(arr, (0..=10).collect::<Vec<_>>());

        let mut arr = (0..=10).rev().collect::<Vec<_>>();
        radix_sort(&mut arr);
        assert_eq!(arr, (0..=10).collect::<Vec<_>>());

        let mut arr = [666666, 55555, 4444, 333, 22, 1];
        radix_sort(&mut arr);
        assert_eq!(arr, [1, 22, 333, 4444, 55555, 666666]);

        let mut arr = [55, 3, 91, 47, 8, 15, 12, 8, 66];
        radix_sort(&mut arr);
        assert_eq!(arr, [3, 8, 8, 12, 15, 47, 55, 66, 91]);

        let mut arr = [2310, 1230, 694, 222, 33, 4, 5566, 918, 77];
        radix_sort(&mut arr);
        assert_eq!(arr, [4, 33, 77, 222, 694, 918, 1230, 2310, 5566]);
    }

    #[test]
    fn test_edge_cases() {
        let mut arr = [];
        radix_sort(&mut arr);
        assert_eq!(arr, []);

        let mut arr = [0];
        radix_sort(&mut arr);
        assert_eq!(arr, [0]);

        let mut arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        radix_sort(&mut arr);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut arr = [u32::MAX, u32::MIN];
        radix_sort(&mut arr);
        assert_eq!(arr, [u32::MIN, u32::MAX]);
    }
}

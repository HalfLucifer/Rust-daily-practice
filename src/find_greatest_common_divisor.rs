pub fn find_gcd(arr: &[u32]) -> u32 {
    if arr.is_empty() {
        return 0;
    }

    let mut result = arr[0];

    for i in 1..arr.len() {
        result = greatest_common_divisor(result, arr[i]);

        if result == 1 {
            break;
        }
    }

    result
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }

    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let arr = [16, 24, 32, 40, 80];
        assert_eq!(find_gcd(&arr), 8);

        let arr = [7, 13, 19, 29, 31];
        assert_eq!(find_gcd(&arr), 1);

        let arr = (1..10).collect::<Vec<_>>();
        assert_eq!(find_gcd(&arr), 1);

        let arr = (1..10).map(|v| v * 2).collect::<Vec<_>>();
        assert_eq!(find_gcd(&arr), 2);

        let arr = (1..10).map(|v| v * 5).collect::<Vec<_>>();
        assert_eq!(find_gcd(&arr), 5);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_gcd(&[]), 0);
        assert_eq!(find_gcd(&[0]), 0);
        assert_eq!(find_gcd(&[0, 1]), 0);
        assert_eq!(find_gcd(&[1, 1]), 1);
        assert_eq!(find_gcd(&[u32::MIN, u32::MAX]), 0);
    }
}

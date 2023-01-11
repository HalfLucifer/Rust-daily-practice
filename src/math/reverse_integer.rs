pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut rev: i64 = 0;

    while x != 0 {
        rev = rev * 10 + (x % 10) as i64;
        x /= 10;
    }

    if rev < i32::MIN as i64 || rev > i32::MAX as i64 {
        return 0;
    }

    rev as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(987654321), 123456789);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(1), 1);
        assert_eq!(reverse(i32::MIN), 0);
        assert_eq!(reverse(i32::MAX), 0);
    }
}

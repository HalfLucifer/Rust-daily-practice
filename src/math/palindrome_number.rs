pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut y = x;
    let mut rev = 0;

    while y != 0 {
        rev = rev * 10 + y % 10;
        y /= 10;
    }

    x == rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(123321), true);
        assert_eq!(is_palindrome(987656789), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(i32::MIN), false);
    }
}

pub fn is_palindrome(s: &str) -> bool {
    let len = s.len() as i32;
    let mut left = 0_i32;
    let mut right = len - 1;
    let input = s.chars().collect::<Vec<char>>();

    while left < right {
        // Skip non-alphanumeric characters
        while left < len as i32 && !input[left as usize].is_ascii_alphanumeric() {
            left += 1;
        }

        // Skip non-alphanumeric characters
        while right > -1 && !input[right as usize].is_ascii_alphanumeric() {
            right -= 1;
        }

        if left == len || right == -1 {
            break;
        }

        if input[left as usize].to_ascii_lowercase() != input[right as usize].to_ascii_lowercase() {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama"), true);
        assert_eq!(is_palindrome("dddccbabccddd"), true);
        assert_eq!(is_palindrome("Aa"), true);

        assert_eq!(is_palindrome("race a car"), false);
        assert_eq!(is_palindrome("0P"), false);
        assert_eq!(is_palindrome("1.1.0"), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_palindrome(""), true);
        assert_eq!(is_palindrome(" "), true);
        assert_eq!(is_palindrome("."), true);

        assert_eq!(is_palindrome(".,"), true);
        assert_eq!(is_palindrome("a."), true);
        assert_eq!(is_palindrome(".a"), true);
    }
}

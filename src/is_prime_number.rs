pub fn is_prime_number(n: u64) -> bool {
    match n {
        0 | 1 => false,
        _ => (2..=(n as f32).sqrt() as u64).all(|v| n % v != 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_prime_number(0), false);
        assert_eq!(is_prime_number(1), false);
        assert_eq!(is_prime_number(u64::MAX), false);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime_number(2), true);
        assert_eq!(is_prime_number(19), true);
        assert_eq!(is_prime_number(53), true);
        assert_eq!(is_prime_number(79), true);
        assert_eq!(is_prime_number(101), true);
        assert_eq!(is_prime_number(977), true);
        assert_eq!(is_prime_number(2017), true);
    }

    #[test]
    fn test_is_not_prime() {
        assert_eq!(is_prime_number(4), false);
        assert_eq!(is_prime_number(21), false);
        assert_eq!(is_prime_number(50), false);
        assert_eq!(is_prime_number(102), false);
        assert_eq!(is_prime_number(501), false);
        assert_eq!(is_prime_number(979), false);
        assert_eq!(is_prime_number(2023), false);
    }
}

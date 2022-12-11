/*
  Bottom-up optimized DP method
  - time complexity O(n)
  - space complexity O(1)
*/
pub fn fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut f0 = 0;
    let mut f1 = 1;
    let mut result = f0 + f1;

    for _ in 2..=n as usize {
        result = f0 + f1;
        f0 = f1;
        f1 = result;
    }

    result
}

/*
   Bottom-up memoized DP method
   - time complexity O(n)
   - space complexity O(n)
*/
pub fn fibonacci_number_dp(n: u32) -> u32 {
    let n = n as usize;
    let mut dp = Vec::with_capacity(n);

    dp.push(0);
    dp.push(1);

    for i in 2..=n {
        dp.push(dp[i - 1] + dp[i - 2]);
    }

    dp[n]
}

/*
   Top-down recursive method
*/
pub fn fibonacci_number_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_number_recursive(n - 1) + fibonacci_number_recursive(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_optimized() {
        assert_eq!(fibonacci_number(0), 0);
        assert_eq!(fibonacci_number(1), 1);
        assert_eq!(fibonacci_number(2), 1);
        assert_eq!(fibonacci_number(10), 55);
        assert_eq!(fibonacci_number(20), 6765);
    }

    #[test]
    fn test_fibonacci_dp() {
        assert_eq!(fibonacci_number_dp(0), 0);
        assert_eq!(fibonacci_number_dp(1), 1);
        assert_eq!(fibonacci_number_dp(2), 1);
        assert_eq!(fibonacci_number_dp(10), 55);
        assert_eq!(fibonacci_number_dp(20), 6765);
    }

    #[test]
    fn test_fibonacci_recursive() {
        assert_eq!(fibonacci_number_recursive(0), 0);
        assert_eq!(fibonacci_number_recursive(1), 1);
        assert_eq!(fibonacci_number_recursive(2), 1);
        assert_eq!(fibonacci_number_recursive(10), 55);
        assert_eq!(fibonacci_number_recursive(20), 6765);
    }
}

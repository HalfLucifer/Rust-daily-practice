/*
    Given a string s, return the number of palindromic substrings in it.
*/
pub fn count_substrings(s: String) -> i32 {
    let arr = s.as_bytes();
    let mut res = 0;

    // DP table: dp[i][j] is true if s[i..=j] is a palindrome
    let mut dp = vec![vec![false; s.len()]; s.len()];

    // Base case: dp[i][i]=true, handled in below case 1)

    // DP table depends on dp[i+1][j-1], so it has to be iterated reversely
    for i in (0..s.len()).rev() {
        for j in i..s.len() {
            // s[i..=j] is a palindrome if s[i]==s[j] and
            //   case 1) string length is 1 or 2
            //   case 2) dp[i+1][j-1] is a palindrome
            if arr[i] == arr[j] && (j - i < 2 || dp[i + 1][j - 1]) {
                dp[i][j] = true;
                res += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(count_substrings("abc".to_owned()), 3);
        assert_eq!(count_substrings("aaa".to_owned()), 6);
        assert_eq!(count_substrings("cbabc".to_owned()), 7);
        assert_eq!(count_substrings("aaaaa".to_owned()), 15);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(count_substrings("0".to_owned()), 1);
        assert_eq!(count_substrings("01".to_owned()), 2);
    }
}

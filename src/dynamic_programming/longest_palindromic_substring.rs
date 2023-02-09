/*
    Given a string s, return the longest palindromic substring in s.
*/

/*
    DP method
    - Time complexity: O(n^2)
    - Space complexity: O(n^2)
*/
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let arr = s.as_bytes();
    let mut res_len = 0;
    let mut res = (0, 0);

    // DP table: dp[i][j]: whether s[i..=j] is a palindrome or not
    let mut dp = vec![vec![false; n]; n];

    // Base case
    for i in 0..n {
        dp[i][i] = true;
    }

    // DP table depends on dp[i+1][j-1], so it has to be iterated reversely
    for i in (0..n).rev() {
        for j in i + 1..n {
            // Found the same chars 
            if arr[i] == arr[j] {
                // s[i..=j] is a palindrome if either substring length is 2 or dp[i+1][j-1] is a palindrome
                if j == i + 1 || dp[i + 1][j - 1] {
                    dp[i][j] = true;

                    if res_len < j - i {
                        res_len = j - i;
                        res = (i, j);
                    }
                }
            }
        }
    }

    s[res.0..=res.1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(longest_palindrome("babad".to_string()), "aba");
        assert_eq!(longest_palindrome("asdfdsarty".to_string()), "asdfdsa");
        assert_eq!(longest_palindrome("aacabdkacaa".to_string()), "aca");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(longest_palindrome("0".to_string()), "0");
        assert_eq!(longest_palindrome("01".to_string()), "0");
        assert_eq!(longest_palindrome("010".to_string()), "010");
    }
}

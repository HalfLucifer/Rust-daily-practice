/*
    Given a string s, return the longest palindromic substring in s.
*/

/*
    Two Pointers method
    - Time complexity: O(n^2)
    - Space complexity: O(1)
*/
pub fn longest_palindrome(s: String) -> String {
    let mut res = (0, 0);
    
    for i in 0..s.len() - 1 {
        // Check for string with odd length
        let p1 = find_palindrome(&s, i as isize, i as isize);        
        res = std::cmp::max_by(res, p1, |p, q| (p.1 - p.0).cmp(&(q.1 - q.0)));

        // Check for string with even length
        let p2 = find_palindrome(&s, i as isize, i as isize + 1);
        res = std::cmp::max_by(res, p2, |p, q| (p.1 - p.0).cmp(&(q.1 - q.0)));
    }
    
    s[res.0 as usize..=res.1 as usize].to_string()
}

fn find_palindrome(s: &str, mut left: isize, mut right: isize) -> (isize, isize) {
    let slice = s.as_bytes();
    
    while left >=0 && right < s.len() as isize && slice[left as usize] == slice[right as usize] {
        // Move from middle to left & right
        left -= 1;
        right += 1
    }

    // Move one step backward
    (left + 1, right - 1)
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

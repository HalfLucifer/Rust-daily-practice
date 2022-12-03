/*
   Criteria: input string contains A-Z and a-z only
*/
pub fn min_window_substring(s: &str, t: &str) -> String {
    let mut result = (0_usize, usize::MAX);
    let mut freq = vec![0; 128];
    let mut left = 0;
    let mut right = 0;
    let mut counter = t.len();
    let arr = s.as_bytes();

    for c in t.chars() {
        freq[(c as u8 - 'A' as u8) as usize] += 1;
    }

    while right < s.len() {
        let curr = (arr[right] as u8 - 'A' as u8) as usize;
        if freq[curr] > 0 {
            counter -= 1;
        }

        freq[curr] -= 1;
        right += 1;

        while counter == 0 {
            if result.1 - result.0 > right - left {
                result.0 = left;
                result.1 = right;
            }

            let prev = (arr[left] as u8 - 'A' as u8) as usize;
            freq[prev] += 1;

            if freq[prev] > 0 {
                counter += 1;
            }

            left += 1;
        }
    }

    if result.1 == usize::MAX {
        "".to_string()
    } else {
        s[result.0..result.1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(min_window_substring("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(min_window_substring("sdszmqxyyzzxskyqwz", "xyz"), "xyyz");
        assert_eq!(min_window_substring("abbbbbcccbbbaaaaaabc", "abc"), "abc");
        assert_eq!(min_window_substring("MNOPQ", "MNOPQ"), "MNOPQ");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(min_window_substring("", ""), "");
        assert_eq!(min_window_substring("a", "a"), "a");
        assert_eq!(min_window_substring("a", "b"), "");
        assert_eq!(min_window_substring("a", "aaaaaa"), "");
    }
}

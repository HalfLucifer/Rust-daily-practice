use std::collections::HashMap;

pub fn longest_nonrepeating_substring(s: &str) -> usize {
    let mut hm: HashMap<char, u32> = HashMap::new();
    let mut left = 0;
    let mut right = 0;
    let mut result = 0;

    while right < s.len() {
        let right_char = s.chars().nth(right).unwrap();
        right += 1;
        hm.entry(right_char).and_modify(|e| *e += 1).or_insert(1);

        while hm.get(&right_char) > Some(&1) {
            let left_char = s.chars().nth(left).unwrap();
            left += 1;
            hm.entry(left_char).and_modify(|e| *e -= 1);
        }

        result = result.max(right - left);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(longest_nonrepeating_substring("aabab"), 2);
        assert_eq!(longest_nonrepeating_substring("abccba"), 3);
        assert_eq!(longest_nonrepeating_substring("abcdabcd"), 4);
        assert_eq!(longest_nonrepeating_substring("aabbccddee"), 2);
        assert_eq!(longest_nonrepeating_substring("abcdefghijklmnopqrstuvwxyz"), 26);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(longest_nonrepeating_substring(""), 0);
        assert_eq!(longest_nonrepeating_substring("1"), 1);
        assert_eq!(longest_nonrepeating_substring("11111111111"), 1);
        assert_eq!(longest_nonrepeating_substring("0101010101010"), 2);
    }
}

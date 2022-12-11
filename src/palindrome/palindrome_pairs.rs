use std::collections::BTreeSet;
use std::collections::HashMap;

pub fn palindrome_pairs(words: Vec<&str>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut hm = HashMap::new();
    let mut bs = BTreeSet::new();

    for i in 0..words.len() {
        // word to index HashMap
        hm.insert(words[i].to_owned(), i);

        // word length BTreeSet (ascending sorted)
        bs.insert(words[i].len());
    }

    for i in 0..words.len() {
        let curr = words[i];
        let curr_len = curr.len();
        let curr_index = *hm.get(curr).unwrap();
        let reversed = words[i].chars().rev().collect::<String>();

        // Check equal length words
        if let Some(&j) = hm.get(&reversed) {
            if i != j {
                result.push(vec![i as i32, j as i32]);
            }
        }

        for k in bs.iter() {
            let next_len = *k;

            // Greater or equal length words are already checked
            if curr_len == next_len {
                break;
            }

            // Given word=[left][mid], check whether [mid] is palindrome
            if is_palindrome(&curr, next_len, curr_len - 1) {
                // If [mid] is palindrome, [right] is [left] reversed
                let right = &curr[..next_len].chars().rev().collect::<String>();

                if let Some(&found) = hm.get(right) {
                    result.push(vec![curr_index as i32, found as i32]);
                }
            }

            // Given word=[mid][right], check whether [mid] is palindrome
            if is_palindrome(&curr, 0, curr_len - next_len - 1) {
                // If [mid] is palindrome, [left] is [right] reversed
                let left = &curr[curr_len - next_len..].chars().rev().collect::<String>();

                if let Some(&found) = hm.get(left) {
                    result.push(vec![found as i32, curr_index as i32]);
                }
            }
        }
    }

    result
}

fn is_palindrome(s: &str, left: usize, right: usize) -> bool {
    let mut left = left;
    let mut right = right;

    while left < right {
        if s[left..left + 1] != s[right..right + 1] {
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
        assert_eq!(
            palindrome_pairs(vec!["abcd", "dcba", "lls", "s", "sssll"]),
            vec![[0, 1], [1, 0], [3, 2], [2, 4]]
        );

        assert_eq!(
            palindrome_pairs(vec!["bat", "tab", "cat"]),
            vec![[0, 1], [1, 0]]
        );

        assert_eq!(
            palindrome_pairs(vec!["a", "b", "c", "ab", "ac", "aa"]),
            vec![[3, 0], [1, 3], [4, 0], [2, 4], [5, 0], [0, 5]]
        );
    }

    #[test]
    fn test_edge_cases() {
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(palindrome_pairs(vec![""]), expected);
        assert_eq!(palindrome_pairs(vec!["a"]), expected);
        assert_eq!(palindrome_pairs(vec!["a", ""]), vec![[0, 1], [1, 0]]);
    }
}

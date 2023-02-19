/*
    Given two strings s and t, return true if t is an anagram of s, and false otherwise.
 */
pub fn is_anagram(s: String, t: String) -> bool {
    let mut dict = vec![0; 26];

    for c in s.chars() {
        dict[(c as u8 - b'a') as usize] += 1;
    }

    for c in t.chars() {
        dict[(c as u8 - b'a') as usize] -= 1;
    }

    dict.iter().all(|i| *i == 0)
}

pub fn is_anagram_by_sorting(s: String, t: String) -> bool {
    let mut v = s.chars().collect::<Vec<char>>();
    let mut w = t.chars().collect::<Vec<char>>();
    
    v.sort_unstable();
    w.sort_unstable();
    
    v == w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(is_anagram("anagram".to_owned(), "nagaram".to_owned()), true);
        assert_eq!(is_anagram("rat".to_owned(), "car".to_owned()), false);
    
        assert_eq!(is_anagram_by_sorting("anagram".to_owned(), "nagaram".to_owned()), true);
        assert_eq!(is_anagram_by_sorting("rat".to_owned(), "car".to_owned()), false);    
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_anagram("a".to_owned(), "a".to_owned()), true);
        assert_eq!(is_anagram("a".to_owned(), "b".to_owned()), false);

        assert_eq!(is_anagram_by_sorting("a".to_owned(), "a".to_owned()), true);
        assert_eq!(is_anagram_by_sorting("a".to_owned(), "b".to_owned()), false);
    }
}

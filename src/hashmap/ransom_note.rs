/*
    Check whether ransom note can be constructed with magazine
*/
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    // Optimization: use char array to replace hash map
    let mut chars = vec![0; 26];

    magazine.chars().for_each(|c| {
        chars[(c.to_ascii_lowercase() as u8 - b'a') as usize] += 1;
    });

    for c in ransom_note.chars() {
        let index = (c as u8 - b'a') as usize;

        if chars[index] == 0 {
            return false;
        }

        chars[index] -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(can_construct("a".to_owned(), "b".to_owned()), false);
        assert_eq!(can_construct("aa".to_owned(), "ab".to_owned()), false);
        assert_eq!(can_construct("aa".to_owned(), "aab".to_owned()), true);

        let s = ('a'..='z').collect::<String>();
        assert_eq!(can_construct(s.to_owned(), "abcdefghijklmnopqrstuvwxy".to_owned()),false);
        assert_eq!(can_construct("aa".to_owned(), s.to_owned()), false);
        assert_eq!(can_construct(s.to_owned(), s.to_owned()), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(can_construct("".to_owned(), "".to_owned()), true);
        assert_eq!(can_construct("".to_owned(), "a".to_owned()), true);
        assert_eq!(can_construct("a".to_owned(), "".to_owned()), false);
    }
}

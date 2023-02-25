/*
   Given a string array words, return an array of all characters that show up in all
   strings within the words (including duplicates).
*/
pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut res = vec![];
    let mut hash = vec![0; 26];

    for c in words[0].chars() {
        hash[(c as u8 - b'a') as usize] += 1;
    }

    for s in words.iter().skip(1) {
        let mut dict = vec![0; 26];

        for c in s.chars() {
            dict[(c as u8 - b'a') as usize] += 1;
        }

        // Find the min occurrence of each character
        for i in 0..26 {
            hash[i] = hash[i].min(dict[i]);
        }
    }

    for i in 0..26 {
        while hash[i] > 0 {
            res.push(((i as u8 + b'a') as char).to_string());
            hash[i] -= 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            common_chars(vec![
                "bella".to_owned(),
                "label".to_owned(),
                "roller".to_owned()
            ]),
            ["e", "l", "l"]
        );

        assert_eq!(
            common_chars(vec![
                "cool".to_owned(),
                "lock".to_owned(),
                "cook".to_owned()
            ]),
            ["c", "o"]
        );
        
        assert_eq!(
            common_chars(vec!["lll".to_owned(), "ll".to_owned(), "l".to_owned()]),
            ["l"]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(common_chars(vec!["a".to_owned()]), ["a"]);
    }
}

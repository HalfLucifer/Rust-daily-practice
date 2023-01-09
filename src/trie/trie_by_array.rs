/*
    Trie data structure implemented by array, which is only used for 
    lowercase alphabetic words
*/
#[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn insert(&mut self, word: String) {
        let mut curr = self;

        for c in word.chars() {
            let i = (c as u8 - b'a') as usize;

            // method 1) create a Trie if it is None
            if curr.children[i].is_none() {
                curr.children[i] = Some(Box::new(Trie::new()));
            }
            curr = curr.children[i].as_mut().unwrap();

            // method 2) use get_or_insert_with
            // curr = curr.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }

        curr.is_end = true;
    }
    
    pub fn search(&self, word: String) -> bool {
        let mut curr = self;

        for c in word.chars() {
            let i = (c as u8 - b'a') as usize;

            if let Some(node) = curr.children[i].as_ref() {
                curr = node;
            } else {
                return false;
            }
        }
        
        curr.is_end
    }
   
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;

        for c in prefix.chars() {
            let i = (c as u8 - b'a') as usize;

            if let Some(node) = curr.children[i].as_ref() {
                curr = node;
            } else {
                return false;
            }
        }
        
        true
    }
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut t = Trie::new();
        let words = ["the", "this", "that", "those", "these", "thou", "who"];

        words.iter().for_each(|s| {
            t.insert(s.to_string());
        });

        words.iter().for_each(|s| {
            assert_eq!(t.search(s.to_string()), true);
        });
    }

    #[test]
    fn test_incompleted_word_cases() {
        let mut t = Trie::new();

        t.insert("apple".to_owned());
        assert_eq!(t.search("a".to_owned()), false);
        assert_eq!(t.search("ap".to_owned()), false);
        assert_eq!(t.search("app".to_owned()), false);
        assert_eq!(t.search("appl".to_owned()), false);
        assert_eq!(t.search("apple".to_owned()), true);

        t.insert("app".to_owned());
        assert_eq!(t.search("a".to_owned()), false);
        assert_eq!(t.search("ap".to_owned()), false);
        assert_eq!(t.search("app".to_owned()), true);

        assert_eq!(t.starts_with("a".to_owned()), true);
        assert_eq!(t.starts_with("ap".to_owned()), true);
        assert_eq!(t.starts_with("app".to_owned()), true);
    }

    #[test]
    fn test_edge_cases() {
        let mut t = Trie::new();
        assert_eq!(t.search("".to_owned()), false);
        assert_eq!(t.search("a".to_owned()), false);

        t.insert("a".to_owned());
        assert_eq!(t.search("a".to_owned()), true);
    }
}

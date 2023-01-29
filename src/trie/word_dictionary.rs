#[derive(Default)]
pub struct WordDictionary {
    root: Trie,
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;

        for &c in word.as_bytes() {
            node = node.children[(c as u8 - b'a') as usize].get_or_insert(Box::new(Default::default()));
        }

        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        Self::search_trie(&self.root, &word.as_bytes())
    }

    fn search_trie(curr: &Trie, word: &[u8]) -> bool {
        if word.len() == 0 {
            return curr.is_end;
        }

        let c = word[0];
        if c == b'.' {
            // When matching '.', need to check every child
            for i in 0..26 {
                if let Some(node) = &curr.children[i] {
                    if Self::search_trie(&node, &word[1..]) {
                        return true;
                    }
                }
            }
        } else {
            // Normal case: matching children recursively
            if let Some(node) = &curr.children[(c - b'a') as usize] {
                return Self::search_trie(&node, &word[1..]);
            }
        }

        false
    }
}

// Trie implemented by array
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".into());
        wd.add_word("dad".into());
        wd.add_word("mad".into());

        assert_eq!(wd.search("pad".into()), false);
        assert_eq!(wd.search("bad".into()), true);
        assert_eq!(wd.search(".ad".into()), true);
        assert_eq!(wd.search("b..".into()), true);
    }

    #[test]
    fn test_edge_cases() {
        let mut wd = WordDictionary::new();
        wd.add_word("a".into());

        assert_eq!(wd.search("a".into()), true);
        assert_eq!(wd.search(".".into()), true);
        assert_eq!(wd.search(".a".into()), false);
        assert_eq!(wd.search("a.".into()), false);
    }
}

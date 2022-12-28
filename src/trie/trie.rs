use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current = &mut self.root;

        for c in word.chars() {
            let next = current.children.entry(c).or_insert(TrieNode::new());
            current = next;
        }

        current.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current = &self.root;

        for c in word.chars() {
            if let Some(next) = current.children.get(&c) {
                current = next;
            } else {
                return false;
            }
        }

        current.is_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;

        for c in prefix.chars() {
            if let Some(next) = current.children.get(&c) {
                current = next;
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
        assert_eq!(t.search("A".to_owned()), false);
    }
}

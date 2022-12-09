use std::collections::HashMap;

struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(is_end_of_word: bool) -> Self {
        Self {
            is_end_of_word: is_end_of_word,
            children: HashMap::new(),
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(false),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut input = word.chars().rev().collect::<Vec<_>>();

        fn insert_recursive(parent: &mut TrieNode, input: &mut Vec<char>) {
            let c = input.pop().unwrap();

            match parent.children.get_mut(&c) {
                Some(node) => {
                    if input.is_empty() {
                        node.is_end_of_word = true;
                    } else {
                        insert_recursive(node, input);
                    }
                }

                None => {
                    parent.children.insert(c, TrieNode::new(input.is_empty()));

                    if !input.is_empty() {
                        insert_recursive(parent.children.get_mut(&c).unwrap(), input);
                    }
                }
            }
        }

        insert_recursive(&mut self.root, &mut input);
    }

    pub fn search(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        let mut input = word.chars().rev().collect::<Vec<_>>();

        fn search_recursive(parent: &TrieNode, input: &mut Vec<char>) -> bool {
            let c = input.pop().unwrap();

            if let Some(node) = parent.children.get(&c) {
                if node.is_end_of_word {
                    return true;
                } else {
                    if input.is_empty() {
                        return false;
                    } else {
                        return search_recursive(node, input);
                    }
                }
            }

            false
        }

        search_recursive(&self.root, &mut input)
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
            t.insert(*s);
        });

        words.iter().for_each(|s| {
            assert_eq!(t.search(*s), true);
        });
    }

    #[test]
    fn test_incompleted_word_cases() {
        let mut t = Trie::new();

        t.insert("apple");
        assert_eq!(t.search("a"), false);
        assert_eq!(t.search("ap"), false);
        assert_eq!(t.search("app"), false);
        assert_eq!(t.search("appl"), false);
        assert_eq!(t.search("apple"), true);

        t.insert("app");
        assert_eq!(t.search("a"), false);
        assert_eq!(t.search("ap"), false);
        assert_eq!(t.search("app"), true);
    }

    #[test]
    fn test_edge_cases() {
        let mut t = Trie::new();

        assert_eq!(t.search(""), false);
        assert_eq!(t.search("a"), false);
        t.insert("a");
        assert_eq!(t.search("a"), true);
        assert_eq!(t.search("A"), false);
    }
}

use std::collections::HashMap;
use std::collections::HashSet;

pub struct FileSystem {
    dirs: HashMap<String, HashSet<String>>,
    files: HashMap<String, String>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn ls(&self, path: String) -> Vec<String> {
        // Check whether it is a file (already in HashMap)
        if self.files.contains_key(&path) {
            // Get file started from the last '/'
            let file = path.split('/').last().unwrap().to_string();
            return vec![file];
        }

        // Otherwise it is a directory
        if let Some(set) = self.dirs.get(&path) {
            return set.iter().cloned().collect();
        }

        vec![]
    }

    pub fn mkdir(&mut self, path: String) {
        let dir = path.split('/').collect::<Vec<_>>();

        for i in 0..dir.len() - 1 {
            let mut curr_path = dir[..i + 1].join("/");
            curr_path.push('/'); // Padding '/' to end of directory

            let sub_path = dir[i + 1].to_string();
            if !sub_path.is_empty() {
                self.dirs.entry(curr_path).or_insert(HashSet::new()).insert(sub_path);
            }
        }
    }

    pub fn add_content_to_file(&mut self, file_path: String, content: String) {
        // Find the index of last '/'
        if let Some(i) = file_path.chars().collect::<Vec<_>>().iter().rposition(|c| *c == '/') {
            // Separate the path and file
            let path = file_path[..i + 1].to_string();
            let file = file_path[i + 1..].to_string();

            // Create the directory if it is not existed
            if !self.dirs.contains_key(&path) {
                self.mkdir(path.clone());
            }

            // Map directory to the file
            self.dirs.entry(path).or_insert(HashSet::new()).insert(file);

            // Map file to the content (append if it is already existed)
            self.files.entry(file_path).and_modify(|s| s.push_str(&content)).or_insert(content);
        }
    }

    pub fn read_content_from_file(&self, file_path: String) -> Option<String> {
        self.files.get(&file_path).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut fs = FileSystem::new();
        assert_eq!(fs.ls("/".into()), Vec::<String>::new());

        fs.mkdir("/a/b/c".into());
        fs.add_content_to_file("/a/b/c/d".into(), "hello".into());

        assert_eq!(fs.ls("/".into()), vec!["a".to_owned()]);
        assert_eq!(fs.ls("/a/".into()), vec!["b".to_owned()]);
        assert_eq!(fs.ls("/a/b/".into()), vec!["c".to_owned()]);
        assert_eq!(fs.ls("/a/b/c/".into()), vec!["d".to_owned()]);
        assert_eq!(
            fs.read_content_from_file("/a/b/c/d".into()),
            Some("hello".to_owned())
        );
    }

    #[test]
    fn test_edge_cases() {
        let fs = FileSystem::new();
        assert_eq!(fs.ls("/".into()), Vec::<String>::new());
        assert_eq!(fs.read_content_from_file("/a/".into()), None);
        assert_eq!(fs.read_content_from_file("//".into()), None);
    }

    #[test]
    fn test_dir_and_file_cases() {
        let mut fs = FileSystem::new();
        fs.mkdir("/a/b".into());
        fs.add_content_to_file("/a/c".into(), "I'm a file".into());

        let mut res = fs.ls("/a/".into());
        res.sort();

        assert_eq!(res, vec!["b".to_owned(), "c".to_owned()]);
        assert_eq!(
            fs.read_content_from_file("/a/c".into()),
            Some("I'm a file".to_owned())
        );
    }

    #[test]
    fn test_add_content_cases() {
        let mut fs = FileSystem::new();
        fs.add_content_to_file("/a/b/c/d".into(), "Hello".into());
        fs.add_content_to_file("/a/b/c/d".into(), " Rust!".into());

        assert_eq!(fs.ls("/".into()), vec!["a".to_owned()]);
        assert_eq!(fs.ls("/a/".into()), vec!["b".to_owned()]);
        assert_eq!(fs.ls("/a/b/".into()), vec!["c".to_owned()]);
        assert_eq!(fs.ls("/a/b/c/".into()), vec!["d".to_owned()]);
        assert_eq!(
            fs.read_content_from_file("/a/b/c/d".into()),
            Some("Hello Rust!".to_owned())
        );
    }
}

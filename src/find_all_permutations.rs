/*
    Limitation: input string can not be composed of any repeated character 
 */
pub fn find_all_permutations(s: &str) -> Vec<String> {
    let mut result = vec![];
    let mut track = String::new();
    backtrack(s, &mut result, &mut track);
    result
}

fn backtrack(s: &str, result: &mut Vec<String>, track: &mut String) {
    if track.len() == s.len() {
        result.push(track.to_string());
        return;
    }

    s.chars().for_each(|c|{
        if !track.contains(c) {
            track.push(c);
            backtrack(s, result, track);
            track.pop();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let ans = vec!["joe", "jeo", "oje", "oej", "ejo", "eoj"];
        let result = find_all_permutations("joe");
        assert!(ans.iter().zip(result.iter()).all(|(x, y)|x == y));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_all_permutations(""), vec![""]);
        assert_eq!(find_all_permutations("0"), vec!["0"]);
    }

    #[test]
    fn test_failed_cases() {
        let v: Vec<String> = vec![];
        assert_eq!(find_all_permutations("oops"), v);
    }
}
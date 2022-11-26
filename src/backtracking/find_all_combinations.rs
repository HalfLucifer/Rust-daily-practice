pub fn find_all_combinations(s: &str, count: u32) -> Vec<String> {
    let mut result = vec![];
    let mut track = String::new();
    backtrack(s, &mut result, &mut track, count);
    result
}
    
fn backtrack(s: &str, result: &mut Vec<String>, track: &mut String, count: u32) {
    if track.len() == count as usize {
        result.push(track.to_string());
        return;
    }

    s.chars().for_each(|c|{
        if !track.contains(c) {
            track.push(c);
            backtrack(s, result, track, count);
            track.pop();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_cases() {
        assert_eq!(find_all_combinations("joe", 0), vec![""]);
    }

    #[test]
    fn test_comb1() {
        let ans = vec!["j", "o", "e"];
        let result = find_all_combinations("joe", 1);
        assert!(ans.iter().zip(result.iter()).all(|(x, y)|x == y));
    }

    #[test]
    fn test_comb2() {
        let ans = vec!["jo", "je", "oj", "oe", "ej", "eo"];
        let result = find_all_combinations("joe", 2);
        assert!(ans.iter().zip(result.iter()).all(|(x, y)|x == y));
    }

    #[test]
    fn test_comb3() {
        let ans = vec!["joe", "jeo", "oje", "oej", "ejo", "eoj"];
        let result = find_all_combinations("joe", 3);
        assert!(ans.iter().zip(result.iter()).all(|(x, y)|x == y));
    }
}
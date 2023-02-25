/*
    Given n pairs of parentheses, write a function to generate all combinations of 
    well-formed parentheses.
*/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(track: &mut Vec<char>, res: &mut Vec<String>, left: i32, right: i32) {
        // Left & right count is impossible to generate valid parenthesis
        if left < 0 || right < 0 || right < left {
            return;
        }

        // All left & right parentheses are used
        if left == 0 && right == 0 {
            res.push(track.iter().collect::<String>());
            return;
        }

        // Try inserting 1 left parenthese
        track.push('(');
        backtrack(track, res, left - 1, right);
        track.pop();

        // Try inserting 1 right parenthese
        track.push(')');
        backtrack(track, res, left, right - 1);
        track.pop();
    }

    let mut res = vec![];
    let mut track = vec![];
    backtrack(&mut track, &mut res, n, n);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(generate_parenthesis(1), vec!["()"]);
        assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(generate_parenthesis(3), vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(generate_parenthesis(0), vec![""]);
    }
}

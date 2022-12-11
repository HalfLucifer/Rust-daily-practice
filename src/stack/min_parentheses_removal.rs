/*
   Given a string, remove minimum invalid parentheses to make string valid
*/
pub fn remove_min_parentheses(s: &str) -> String {
    let mut stack_left = vec![];
    let mut stack_right = vec![];

    s.char_indices().for_each(|(i, c)| match c {
        '(' => {
            stack_left.push(i);
        }

        ')' => {
            if stack_left.is_empty() {
                stack_right.push(i);
            } else {
                stack_left.pop();
            }
        }

        _ => {}
    });

    s.char_indices()
        .filter(|(i, _)| !stack_left.contains(i) && !stack_right.contains(i))
        .map(|(_, c)| c.to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(remove_min_parentheses("(a))"), "(a)");
        assert_eq!(remove_min_parentheses("((((a)"), "(a)");
        assert_eq!(remove_min_parentheses("a(b)c((d)e(f)))"), "a(b)c((d)e(f))");
    }

    #[test]
    fn test_parentheses_only_cases() {
        assert_eq!(remove_min_parentheses(")))))))(((((((((((((((("), "");
        assert_eq!(
            remove_min_parentheses("()()()()()()()()()()"),
            "()()()()()()()()()()"
        );
        assert_eq!(
            remove_min_parentheses("(())))()(()))))())))))"),
            "(())()(())()"
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(remove_min_parentheses(""), "");
        assert_eq!(remove_min_parentheses("("), "");
        assert_eq!(remove_min_parentheses(")"), "");
        assert_eq!(remove_min_parentheses("(( "), " ");
        assert_eq!(remove_min_parentheses("a"), "a");
    }
}

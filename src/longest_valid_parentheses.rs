pub fn longest_valid_parentheses(s: &str) -> i32 {
    let mut result = 0;
    let mut stack = vec![];

    for (i, c) in s.char_indices() {
        match c {
            '(' => {
                stack.push((i, c));
            }

            ')' => {
                if stack.is_empty() || stack.last().unwrap().1 == ')' {
                    stack.push((i, c));
                } else {
                    stack.pop();

                    if let Some(top) = stack.last() {
                        result = result.max(i - top.0);
                    } else {
                        result = result.max(i + 1);
                    }
                }
            }

            _ => unreachable!(),
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(longest_valid_parentheses("()"), 2);
        assert_eq!(longest_valid_parentheses("(()"), 2);
        assert_eq!(longest_valid_parentheses("(())"), 4);
        assert_eq!(longest_valid_parentheses("(()))"), 4);
        assert_eq!(longest_valid_parentheses("()()()(()))"), 10);
        assert_eq!(
            longest_valid_parentheses(")))()()()))))))((((((()))))))"),
            14
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(longest_valid_parentheses(""), 0);
        assert_eq!(longest_valid_parentheses(")("), 0);
        assert_eq!(longest_valid_parentheses(")))))))))))))))))"), 0);
        assert_eq!(longest_valid_parentheses("((((((((((((((((("), 0);
    }
}

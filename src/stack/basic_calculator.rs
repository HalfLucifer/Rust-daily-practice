pub fn basic_calculator(s: &str) -> i32 {
    // Convert to a mutable string for recursive operation
    basic_calculator_recursive(&mut s.to_string())
}

fn basic_calculator_recursive(input: &mut String) -> i32 {
    let mut stack = vec![];
    let mut number = 0;
    let mut operator = '+';

    while input.len() > 0 {
        let c = input.remove(0);

        // Convert char to number
        if c.is_digit(10) {
            number = number * 10 + (c as u8 - '0' as u8) as i32;
        }

        // Begin recursive call at left parenthese
        if c == '(' {
            number = basic_calculator_recursive(input);
        }

        if (!c.is_digit(10) && c != ' ') || input.len() == 0 {
            match operator {
                '+' => {
                    stack.push(number);
                }

                '-' => {
                    stack.push(-number);
                }

                '*' => {
                    let pre = stack.pop().unwrap_or(0);
                    stack.push(pre * number);
                }

                '/' => {
                    let pre = stack.pop().unwrap_or(0);
                    stack.push(pre / number);
                }

                '(' | ')' => {}

                _ => {
                    panic!("Unexpected operator.");
                }
            }

            operator = c;
            number = 0;
        }

        // End recursive call at right parenthese
        if c == ')' {
            break;
        }
    }

    stack.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(basic_calculator("0+1-2*3/4"), 0 + 1 - 2 * 3 / 4);
        assert_eq!(basic_calculator("0-1*2/3+4"), 0 - 1 * 2 / 3 + 4);
        assert_eq!(basic_calculator("0*1/2+3-4"), 0 * 1 / 2 + 3 - 4);
        assert_eq!(basic_calculator("0/1+2-3*4"), 0 / 1 + 2 - 3 * 4);

        assert_eq!(basic_calculator("7 - 4 + 1 / 3"), 7 - 4 + 1 / 3);
        assert_eq!(basic_calculator("7 - (4 + 1) / 3"), 7 - (4 + 1) / 3);
        assert_eq!(basic_calculator("(7 - (4 + 1)) / 3"), (7 - (4 + 1)) / 3);
        assert_eq!(basic_calculator("7 - 4 + (1 / 3)"), 7 - 4 + (1 / 3));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(basic_calculator("0"), 0);
        assert_eq!(basic_calculator("0+1"), 1);
        assert_eq!(basic_calculator("0-1"), -1);
        assert_eq!(basic_calculator("0*1"), 0);
        assert_eq!(basic_calculator("0/1"), 0);
    }

    #[test]
    fn test_multiple_digits_cases() {
        assert_eq!(basic_calculator("11 + 222 - 333"), 11 + 222 - 333);
        assert_eq!(basic_calculator("1000 - 876 / 4"), 1000 - 876 / 4);
        assert_eq!(basic_calculator("5000 / 20 * 13"), 5000 / 20 * 13);
        assert_eq!(basic_calculator("444 * 77 - 999 / 22"), 444 * 77 - 999 / 22);
        assert_eq!(basic_calculator("9999 - (-9999)"), 9999 - (-9999));
    }
}

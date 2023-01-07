use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let input = s.chars().collect::<Vec<_>>();
    let hm: HashMap<char, i32> = HashMap::from_iter([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut result = 0;
    let mut i = 0;

    while i < input.len() {
        let curr = hm.get(&input[i]).unwrap();

        if i == input.len() - 1 {
            result += curr;
        } else {
            let next = hm.get(&input[i + 1]).unwrap();

            if curr >= next {
                result += curr;
            } else {
                // Handle IV, IX, XL, etc. cases
                result += next - curr;
                i += 1;
            }
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(roman_to_int("III".into()), 3);
        assert_eq!(roman_to_int("LVIII".into()), 58);
        assert_eq!(roman_to_int("MCMXCIV".into()), 1994);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(roman_to_int("I".into()), 1);
        assert_eq!(roman_to_int("M".into()), 1000);
        assert_eq!(roman_to_int("MMMMMMMMMM".into()), 10000);
    }
}

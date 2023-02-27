/*
    Given a string containing digits from 2-9 inclusive, return all possible letter
    combinations that the number could represent. Return the answer in any order.

    A mapping of digits to letters is given below. Note that 1 does not map to any letters.
    - 2 => abc
    - 3 => def
    - 4 => ghi
    - 5 => jkl
    - 6 => mno
    - 7 => pqrs
    - 8 => tuv
    - 9 => wxyz
*/
pub fn letter_combinations(digits: String) -> Vec<String> {
    fn backtrack(input: &Vec<Vec<char>>, track: &mut Vec<char>, res: &mut Vec<String>, start: usize, count: usize) {
        if track.len() == count {
            res.push(track.iter().collect::<String>());
            return;
        }

        for i in start..input.len() {
            for j in 0..input[i].len() {
                track.push(input[i][j]);
                backtrack(input, track, res, i + 1, count);
                track.pop();
            }
        }
    }

    let input = digits
        .chars()
        .map(|c| match c {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![],
        })
        .collect::<Vec<Vec<char>>>();

    if input.is_empty() {
        return vec![];
    }

    let mut res = vec![];
    let mut track = vec![];
    backtrack(&input, &mut track, &mut res, 0, input.len());
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(letter_combinations("2".to_string()), vec!["a", "b", "c"]);
        assert_eq!(letter_combinations("23".to_string()), vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
        assert_eq!(letter_combinations("89".to_string()), vec!["tw", "tx", "ty", "tz", "uw", "ux", "uy", "uz", "vw", "vx", "vy", "vz"]);
        assert_eq!(letter_combinations("456".to_string()), vec!["gjm", "gjn", "gjo", "gkm", "gkn", "gko", "glm", "gln", "glo", "hjm", "hjn", "hjo", "hkm", "hkn", "hko", "hlm", "hln", "hlo", "ijm", "ijn", "ijo", "ikm", "ikn", "iko", "ilm", "iln", "ilo"]);
    }

    #[test]
    fn test_edge_cases() {
        let empty: Vec<String> = vec![];
        assert_eq!(letter_combinations("".to_string()), empty);
        assert_eq!(letter_combinations("0".to_string()), empty);
        assert_eq!(letter_combinations("1".to_string()), empty);
        assert_eq!(letter_combinations("1010".to_string()), empty);
    }
}

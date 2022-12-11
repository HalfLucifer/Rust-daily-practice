const SEP: char = '#';

// Stateless struct
pub struct Codec {}

impl Codec {
    pub fn encode(strs: &Vec<String>) -> String {
        // Encoded format: {len1}#{str1}{len2}#{str2}...
        strs.iter()
            .map(|s| format!("{}{}{}", s.len(), SEP, s))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn decode(input: String) -> Vec<String> {
        let mut result = vec![];
        let mut len = 0;

        for (i, c) in input.char_indices() {
            match c {
                SEP => {
                    if len != 0 {
                        result.push(input[i + 1..=i + len].to_string());
                        len = 0;
                    }
                }

                '0'..='9' => {
                    len = len * 10 + (c as u8 - '0' as u8) as usize;
                }

                _ => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string(input: &[&str]) -> Vec<String> {
        input.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_normal_cases() {
        let arr = to_string(&["hello", "world!"]);
        assert_eq!(Codec::decode(Codec::encode(&arr)), arr);

        let arr = to_string(&["there", "was", "no", "book"]);
        assert_eq!(Codec::decode(Codec::encode(&arr)), arr);

        let arr = to_string(&["what ", " if", "s p a c e", "e    xist?"]);
        assert_eq!(Codec::decode(Codec::encode(&arr)), arr);
    }

    #[test]
    fn test_edge_cases() {
        let arr = to_string(&["a#b", "cde"]);
        assert_eq!(Codec::decode(Codec::encode(&arr)), arr);

        let arr = to_string(&["#", " wha#t", "i#s", "#####goin on?"]);
        assert_eq!(Codec::decode(Codec::encode(&arr)), arr);

        let arr = to_string(&["   #", "  ", "why", " are", "YOU#", "?"]);
        assert_eq!(Codec::decode(Codec::encode(&arr)), arr);
    }
}

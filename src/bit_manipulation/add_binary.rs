/*
   Given two binary strings a and b, return their sum as a binary string.
*/
use std::collections::VecDeque;

pub fn add_binary(a: String, b: String) -> String {
    let av = a.as_bytes();
    let bv = b.as_bytes();

    let mut i = av.len() as isize - 1;
    let mut j = bv.len() as isize - 1;
    let mut carry = 0;
    let mut res = VecDeque::new();

    while i >= 0 || j >= 0 || carry == 1 {
        let d1 = {
            if i >= 0 {
                av[i as usize] - '0' as u8
            } else {
                0
            }
        };

        let d2 = {
            if j >= 0 {
                bv[j as usize] - '0' as u8
            } else {
                0
            }
        };

        let sum = d1 ^ d2 ^ carry;
        carry = (d1 | d2) & (d1 | carry) & (d2 | carry);

        res.push_front((sum + '0' as u8) as char);

        i -= 1;
        j -= 1;
    }

    res.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(add_binary("11".into(), "1".into()), "100");
        assert_eq!(add_binary("1010".into(), "1011".into()), "10101");
        assert_eq!(add_binary("111111".into(), "000000".into()), "111111");
        assert_eq!(add_binary("11111111111111111111111111111101".into(), "1".into()), "11111111111111111111111111111110");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(add_binary("0".into(), "0".into()), "0");
        assert_eq!(add_binary("0".into(), "1".into()), "1");
    }
}

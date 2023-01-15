/*
    Reverse bits of a given 32 bits unsigned integer.
*/
pub fn reverse_bits(x: u32) -> u32 {
    let mut res = 0;
    let mut x = x;

    for _ in 0..32 {
        res = res << 1;

        if x & 1 == 1 {
            res += 1;
        }

        x = x >> 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(reverse_bits(1), 2147483648);
        assert_eq!(reverse_bits(7), 3758096384);
        assert_eq!(reverse_bits(43261596), 964176192);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(reverse_bits(0), 0);
        assert_eq!(reverse_bits(u32::MIN), 0);
        assert_eq!(reverse_bits(u32::MAX), 4294967295);
    }
}

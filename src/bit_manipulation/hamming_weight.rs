/*
    Takes an unsigned integer and returns the number of '1' bits it has 
    (also known as the Hamming weight).
*/
pub fn hamming_weight(n: u32) -> i32 {
    let mut res = 0;
    let mut n = n;

    while n != 0 {
        res += n & 1;
        n = n >> 1;
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(hamming_weight(1), 1);
        assert_eq!(hamming_weight(10), 2);
        assert_eq!(hamming_weight(999), 8);

    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(hamming_weight(0), 0);
        assert_eq!(hamming_weight(u32::MIN), 0);
        assert_eq!(hamming_weight(u32::MAX), 32);
    }
}

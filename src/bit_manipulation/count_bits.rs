/*
    Given an integer n, return an array ans of length n + 1 such that for
    each i (0 <= i <= n), ans[i] is the number of 1's in the binary
    representation of i.
*/
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut res = vec![0; n as usize + 1];

    for i in 1..=n as usize {
        res[i] = res[i >> 1] + (i & 1) as i32;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(count_bits(2), [0, 1, 1]);
        assert_eq!(count_bits(5), [0, 1, 1, 2, 1, 2]);
        assert_eq!(count_bits(10), [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(count_bits(0), [0]);
        assert_eq!(count_bits(1), [0, 1]);
    }
}

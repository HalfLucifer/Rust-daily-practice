/*
   Write an algorithm to determine if a number n is happy.

   A happy number is a number defined by the following process:
   - Starting with any positive integer, replace the number by the sum of the squares of
     its digits.
   - Repeat the process until the number equals 1 (where it will stay), or it loops
     endlessly in a cycle which does not include 1.
   - Those numbers for which this process ends in 1 are happy.

   Return true if n is a happy number, and false if not.
*/
use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut hm = HashSet::new();
    let mut num = n;

    while num != 1 {
        let mut sum = 0;

        while num > 0 {
            sum += (num % 10).pow(2);
            num /= 10;
        }

        num = sum;

        if hm.contains(&num) {
            return false;
        }

        hm.insert(num);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(is_happy(19), true);
        assert_eq!(is_happy(2), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_happy(0), false);
        assert_eq!(is_happy(1), true);
    }
}

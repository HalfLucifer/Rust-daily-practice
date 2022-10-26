/*
   Algorithm: Sieve of Eratosthenes
   https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
*/
pub fn count_primes_soe(n: usize) -> usize {
    if n < 2 {
        return 0;
    }

    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    (2..=n).take_while(|i| i * i <= n).for_each(|i| {
        if is_prime[i] {
            (2..=n).take_while(|j| i * j <= n).for_each(|j| {
                is_prime[i * j] = false;
            });
        }
    });

    is_prime.iter().filter(|v| **v).count()
}

/*
   Brute force method
*/
pub fn count_primes(n: usize) -> usize {
    let mut result = 0;

    (1..=n).for_each(|num| {
        if is_prime_number(num as u64) {
            result += 1;
        }
    });

    result
}

fn is_prime_number(n: u64) -> bool {
    match n {
        0 | 1 => false,
        _ => (2..=(n as f32).sqrt() as u64).all(|v| n % v != 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(count_primes_soe(10), count_primes(10));
        assert_eq!(count_primes_soe(500), count_primes(500));
        assert_eq!(count_primes_soe(3000), count_primes(3000));
        assert_eq!(count_primes_soe(9999), count_primes(9999));
        assert_eq!(count_primes_soe(20000), count_primes(20000));
        assert_eq!(count_primes_soe(65535), count_primes(65535));
        assert_eq!(count_primes_soe(99999), count_primes(99999));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(count_primes_soe(0), 0);
        assert_eq!(count_primes_soe(1), 0);
        assert_eq!(count_primes_soe(2), 1);
    }
}

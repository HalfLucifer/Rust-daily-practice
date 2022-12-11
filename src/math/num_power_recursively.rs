pub fn num_power_recursively(base: u64, power: u32) -> u64 {
    if power == 0 {
        return 1;
    }

    if power % 2 == 0 {
        let half = num_power_recursively(base, power / 2);
        half * half
    } else {
        base * num_power_recursively(base, power - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(num_power_recursively(1, 100), 1);
        assert_eq!(num_power_recursively(2, 50), 2_u64.pow(50));
        assert_eq!(num_power_recursively(3, 40), 3_u64.pow(40));
        
        assert_eq!(num_power_recursively(4, 30), 4_u64.pow(30));
        assert_eq!(num_power_recursively(5, 20), 5_u64.pow(20));
        assert_eq!(num_power_recursively(6, 10), 6_u64.pow(10));

        for i in 0..=10 {
            assert_eq!(num_power_recursively(2, i), 2_u64.pow(i));
            assert_eq!(num_power_recursively(10, i), 10_u64.pow(i));
            assert_eq!(num_power_recursively(i as u64, i), (i as u64).pow(i));
        }
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(num_power_recursively(0, 0), 0_u64.pow(0));
        assert_eq!(num_power_recursively(0, 1), 0_u64.pow(1));

        assert_eq!(num_power_recursively(0, u32::MAX), 0_u64.pow(u32::MAX));
        assert_eq!(num_power_recursively(1, u32::MAX), 1_u64.pow(u32::MAX));

        assert_eq!(num_power_recursively(u64::MAX, 0), u64::MAX.pow(0));
        assert_eq!(num_power_recursively(u64::MAX, 1), u64::MAX.pow(1));
    }
}

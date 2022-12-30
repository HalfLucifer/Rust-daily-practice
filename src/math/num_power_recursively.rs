pub fn num_power_recursively(base: f64, power: i32) -> f64 {
    if base == 0.0 {
        return 0.0;
    } else if base == 1.0 {
        return 1.0;
    } else if base == -1.0 {
        if power % 2 == 0 {
            return 1.0;
        } else {
            return -1.0;
        }
    }

    match power {
        i32::MIN | i32::MAX => 0.0,
        0 => 1.0,
        1 => base,
        _ => {
            let m = power.abs();
            let mut res = num_power_recursively(base, m / 2);

            res *= res;

            if m % 2 == 1 {
                res *= base;
            }

            if power < 0 {
                res = 1.0 / res;
            }

            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(num_power_recursively(1.0, 100), 1.0);
        assert_eq!(num_power_recursively(2.0, 50), 2_f64.powf(50.0));
        assert_eq!(num_power_recursively(3.0, 40), 3_f64.powf(40.0));
        
        assert_eq!(num_power_recursively(4.0, 30), 4_f64.powf(30.0));
        assert_eq!(num_power_recursively(5.0, 20), 5_f64.powf(20.0));
        assert_eq!(num_power_recursively(6.0, 10), 6_f64.powf(10.0));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(num_power_recursively(0.0, 0), 0.0);
        assert_eq!(num_power_recursively(0.0, 1), 0.0);

        assert_eq!(num_power_recursively(0.0, i32::MAX), 0.0);
        assert_eq!(num_power_recursively(0.0, i32::MIN), 0.0);

        assert_eq!(num_power_recursively(1.0, i32::MAX), 1.0);
        assert_eq!(num_power_recursively(1.0, i32::MIN), 1.0);
        
        assert_eq!(num_power_recursively(-1.0, i32::MAX), -1.0);
        assert_eq!(num_power_recursively(-1.0, i32::MIN), 1.0);
    }
}

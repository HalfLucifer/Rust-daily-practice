use std::collections::HashMap;

pub fn min_coin_changer(amount: i32, coins: &[u32]) -> Option<i32> {
    if amount == 0 || coins.is_empty() || coins.contains(&0) {
        return None;
    }

    let mut memo: HashMap<i32, i32> = HashMap::new();
    coin_changer_dp(amount, coins, &mut memo)
}

fn coin_changer_dp(amount: i32, coins: &[u32], memo: &mut HashMap<i32, i32>) -> Option<i32> {
    if let Some(x) = memo.get(&amount) {
        return Some(*x);
    }

    if amount == 0 {
        return Some(0);
    } else if amount < 1 {
        return None;
    }

    let mut result = i32::MAX;
    for c in coins.iter() {
        let remaining = amount - *c as i32;

        if let Some(next_amount) = coin_changer_dp(remaining, coins, memo) {
            result = result.min(1 + next_amount);
            memo.insert(amount, result);
        }
    }

    match result {
        i32::MAX => None,
        _ => Some(result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(Some(5), min_coin_changer(5, &[1]));
        assert_eq!(Some(2), min_coin_changer(6, &[1, 2, 3]));
        assert_eq!(Some(3), min_coin_changer(13, &[1, 3, 5]));
        assert_eq!(Some(6), min_coin_changer(50, &[1, 3, 5, 7, 9]));
    }

    #[test]
    fn test_bignum_cases() {
        assert_eq!(Some(1000), min_coin_changer(1000, &[1]));
        assert_eq!(Some(500), min_coin_changer(1000, &[1, 2]));
        assert_eq!(Some(334), min_coin_changer(1000, &[1, 2, 3]));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(None, min_coin_changer(0, &[]));
        assert_eq!(None, min_coin_changer(1, &[0]));
        assert_eq!(None, min_coin_changer(5, &[2]));
        assert_eq!(None, min_coin_changer(10, &[11, 12, 13, 14, 15]));
    }
}

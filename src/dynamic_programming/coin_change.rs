/*
    Given an integer array coins representing coins of different denominations and
    an integer amount representing a total amount of money. Return the fewest number
    of coins that you need to make up that amount. If that amount of money cannot be
    made up by any combination of the coins, return -1.

    Assume that you have an infinite number of each kind of coin.
*/

// Bottom-up DP method
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // DP table
    //   j: the amount of total coins
    //   dp[j]: the min count of coins sum up to j
    let mut dp = vec![amount + 1; amount as usize + 1];

    // Base case: the min count is 0 for amount 0
    dp[0] = 0;

    for i in 0..coins.len() {
        for j in 1..=amount as usize {
            if j as i32 - coins[i] >= 0 {
                dp[j] = dp[j].min(1 + dp[j - coins[i] as usize])
            }
        }
    }

    if dp[amount as usize] == amount + 1 {
        -1
    } else {
        dp[amount as usize]
    }
}

// Recursive with memo DP method
pub fn coin_change_memo(coins: Vec<i32>, amount: i32) -> Option<i32> {
    use std::collections::HashMap;

    fn coin_changer_dp(amount: i32, coins: &Vec<i32>, memo: &mut HashMap<i32, i32>) -> Option<i32> {
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

            if let Some(next_amount) = coin_changer_dp(remaining, &coins, memo) {
                result = result.min(1 + next_amount);
                memo.insert(amount, result);
            }
        }

        match result {
            i32::MAX => None,
            _ => Some(result),
        }
    }

    if amount == 0 || coins.is_empty() || coins.contains(&0) {
        return None;
    }

    let mut memo = HashMap::new();
    coin_changer_dp(amount, &coins, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(coin_change(vec![1], 5), 5);
        assert_eq!(coin_change(vec![1, 2, 3], 6), 2);
        assert_eq!(coin_change(vec![1, 3, 5], 13), 3);
        assert_eq!(coin_change(vec![1, 3, 5, 7, 9], 50), 6);

        assert_eq!(coin_change(vec![1], 1000), 1000);
        assert_eq!(coin_change(vec![1, 2], 1000), 500);
        assert_eq!(coin_change(vec![1, 2, 3], 1000), 334);
    }

    #[test]
    fn test_memo_cases() {
        assert_eq!(Some(5), coin_change_memo(vec![1], 5));
        assert_eq!(Some(2), coin_change_memo(vec![1, 2, 3], 6));
        assert_eq!(Some(3), coin_change_memo(vec![1, 3, 5], 13));
        assert_eq!(Some(6), coin_change_memo(vec![1, 3, 5, 7, 9], 50));

        assert_eq!(Some(1000), coin_change_memo(vec![1], 1000));
        assert_eq!(Some(500), coin_change_memo(vec![1, 2], 1000));
        assert_eq!(Some(334), coin_change_memo(vec![1, 2, 3], 1000));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(coin_change(vec![0], 1), -1);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![11, 12, 13, 14, 15], 10), -1);

        assert_eq!(None, coin_change_memo(vec![0], 1));
        assert_eq!(None, coin_change_memo(vec![2], 3));
        assert_eq!(None, coin_change_memo(vec![11, 12, 13, 14, 15], 10));
    }
}

use rand::Rng;

/*
  Fisher–Yates shuffle algorithm
  - time complexity O(n)
  - space complexity O(1)
*/
pub fn shuffle_deck(deck: &mut [u32]) {
    if !deck.is_empty() {
        let mut rng = rand::thread_rng();
        
        for i in (1..=deck.len() - 1).rev() {
            let j = rng.gen_range(0..=i);
            deck.swap(j, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut deck = vec![0, 1];
        shuffle_deck(&mut deck);
        assert!(deck == [0, 1] || deck == [1, 0]);

        let mut deck = vec![0, 1, 2];
        shuffle_deck(&mut deck);
        assert!(
            deck == [0, 1, 2]
                || deck == [0, 2, 1]
                || deck == [1, 0, 2]
                || deck == [1, 2, 0]
                || deck == [2, 0, 1]
                || deck == [2, 1, 0]
        );

        let mut deck = (1..=52).collect::<Vec<_>>();
        let expected = deck.clone();
        shuffle_deck(&mut deck);
        deck.sort();
        assert_eq!(deck, expected);
    }

    #[test]
    fn test_edge_cases() {
        let mut deck = vec![];
        shuffle_deck(&mut deck);

        let mut deck = vec![0];
        shuffle_deck(&mut deck);
        assert_eq!(deck, [0]);
    }
}

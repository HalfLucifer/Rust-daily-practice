/*
    Criteria: each input can be used an unlimited number of times
*/
pub fn combination_sum(input: &[u32], sum: u32) -> Vec<Vec<u32>> {
    fn backtrack(
        result: &mut Vec<Vec<u32>>,
        track: &mut Vec<u32>,
        input: &[u32],
        sum: i32,
        start: usize,
    ) {
        if sum <= 0 {
            if sum == 0 {
                result.push(track.clone());
            }
            return;
        }

        for i in start..input.len() {
            let num = input[i];
            track.push(num);
            backtrack(result, track, input, sum - num as i32, i);
            track.pop();
        }
    }

    let mut result = vec![];
    let mut track = vec![];
    backtrack(&mut result, &mut track, input, sum as i32, 0);

    result
}

/*
    Criteria: each input can only be chosen once
*/
pub fn combination_sum_once(input: &[u32], sum: u32) -> Vec<Vec<u32>> {
    fn backtrack(
        result: &mut Vec<Vec<u32>>,
        track: &mut Vec<u32>,
        input: &Vec<u32>,
        sum: i32,
        start: usize,
    ) {
        if sum <= 0 {
            if sum == 0 {
                result.push(track.clone());
            }
            return;
        }

        for i in start..input.len() {
            let num = input[i];

            if i == start || num != input[i - 1] {
                track.push(num);
                backtrack(result, track, input, sum - num as i32, i + 1);
                track.pop();
            }
        }
    }

    let mut result = vec![];
    let mut track = vec![];
    let mut input = input.to_vec();

    input.sort();
    backtrack(&mut result, &mut track, &input, sum as i32, 0);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            combination_sum(&[1, 2, 3], 6),
            [
                vec![1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 2],
                vec![1, 1, 1, 3],
                vec![1, 1, 2, 2],
                vec![1, 2, 3],
                vec![2, 2, 2],
                vec![3, 3]
            ]
        );

        assert_eq!(combination_sum(&[2, 3, 6, 7], 7), [vec![2, 2, 3], vec![7]]);

        assert_eq!(
            combination_sum(&[2, 3, 5], 8),
            [vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );

        assert_eq!(combination_sum(&[3, 5, 8], 11), [vec![3, 3, 5], vec![3, 8]]);

        assert_eq!(
            combination_sum(&[7, 3, 2], 18),
            [
                vec![7, 7, 2, 2],
                vec![7, 3, 3, 3, 2],
                vec![7, 3, 2, 2, 2, 2],
                vec![3, 3, 3, 3, 3, 3],
                vec![3, 3, 3, 3, 2, 2, 2],
                vec![3, 3, 2, 2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2, 2, 2, 2, 2]
            ]
        );
    }

    #[test]
    fn test_input_once_cases() {
        assert_eq!(combination_sum_once(&[1, 2, 3], 6), [vec![1, 2, 3]]);
        assert_eq!(combination_sum_once(&[2, 3, 6, 7], 7), [vec![7]]);
        assert_eq!(combination_sum_once(&[2, 3, 5], 8), [vec![3, 5]]);
        assert_eq!(combination_sum_once(&[3, 5, 8], 11), [vec![3, 8]]);

        let expected: Vec<Vec<u32>> = vec![];
        assert_eq!(combination_sum_once(&[7, 3, 2], 18), expected);

        assert_eq!(
            combination_sum_once(&[10, 1, 2, 7, 6, 1, 5], 8),
            [vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );

        assert_eq!(
            combination_sum_once(&[2, 5, 2, 1, 2], 5),
            [vec![1, 2, 2], vec![5]]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(combination_sum(&[], 0), [vec![]]);
        assert_eq!(combination_sum(&[1], 1), [vec![1]]);
    }
}

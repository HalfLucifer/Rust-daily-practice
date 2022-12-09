pub fn employee_free_time(schedules: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = vec![];
    let mut input = schedules.clone();

    input.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut curr = &input[0];

    for i in 1..input.len() {
        let next = &input[i];

        if curr[1] < next[0] {
            result.push(vec![curr[1], next[0]]);
            curr = next;
        } else {
            if curr[1] < next[1] {
                curr = next;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            employee_free_time(&vec![vec![0, 1], vec![5, 8]]),
            vec![vec![1, 5]]
        );

        assert_eq!(
            employee_free_time(&vec![vec![0, 5], vec![5, 10], vec![11, 20]]),
            vec![vec![10, 11]]
        );

        assert_eq!(
            employee_free_time(&vec![vec![20, 24], vec![15, 21], vec![5, 14]]),
            vec![vec![14, 15]]
        );
    }

    #[test]
    fn test_edge_cases() {
        let mut expected: Vec<Vec<u32>> = vec![vec![]];
        expected.clear();

        assert_eq!(employee_free_time(&vec![vec![0, 0]]), expected);
        assert_eq!(employee_free_time(&vec![vec![0, 100]]), expected);
        
        assert_eq!(
            employee_free_time(&vec![vec![0, 24], vec![1, 2], vec![10, 20]]),
            expected
        );

        let mut schedule = vec![];
        for i in 0..24 {
            schedule.push(vec![i, i + 1]);
        }
        assert_eq!(employee_free_time(&schedule), expected);
    }
}

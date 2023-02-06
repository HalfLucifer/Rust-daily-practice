/*
    Given an array of integers temperatures represents the daily temperatures, return an array
    answer such that answer[i] is the number of days you have to wait after the ith day to get
    a warmer temperature.

    If there is no future day for which this is possible, keep answer[i] == 0 instead.
*/
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack = vec![];

    for i in (0..temperatures.len()).rev() {
        let current = temperatures[i];

        while let Some(&index) = stack.last() {
            if temperatures[index] <= current {
                stack.pop();
            } else {
                break;
            }
        }

        if !stack.is_empty() {
            result[i] = (*stack.last().unwrap() - i) as i32;
        }

        stack.push(i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]), [1, 1, 4, 2, 1, 1, 0, 0]);
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), [1, 1, 1, 0]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(daily_temperatures(vec![0]), [0]);
        assert_eq!(daily_temperatures(vec![i32::MIN, i32::MAX]), [1, 0]);
    }
}

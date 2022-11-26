pub fn merge_intervals(input: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    let mut queue = vec![];

    input.iter().for_each(|(begin, end)| {
        assert!(begin <= end);
        queue.push((*begin, 1));
        queue.push((*end, -1));
    });

    // Sort 'begin' in ascending order and then 'end' in descending order
    queue.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut sum = 0;
    let mut begin = 0;

    queue.into_iter().for_each(|(time, value)| {
        if sum == 0 && value == 1 {
            begin = time;
        } else if sum > 0 && value + sum == 0 {
            let end = time;
            result.push((begin, end));
        }

        sum += value;
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(merge_intervals(&vec![(1, 3), (2, 4)]), [(1, 4)]);
        assert_eq!(merge_intervals(&vec![(2, 7), (3, 4), (5, 6)]), [(2, 7)]);

        assert_eq!(merge_intervals(&vec![(0, 1), (1, 2), (2, 3), (3, 4)]), [(0, 4)]);
        assert_eq!(merge_intervals(&vec![(0, 1), (2, 7), (5, 9)]), [(0, 1), (2, 9)]);

        assert_eq!(merge_intervals(&vec![(50, 100), (0, 10), (10, 51)]), [(0, 100)]);
        assert_eq!(merge_intervals(&vec![(10, 15), (16, 20), (21, 25)]), [(10, 15), (16, 20), (21, 25)]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(merge_intervals(&vec![]), []);
        assert_eq!(merge_intervals(&vec![(0, 0)]), [(0, 0)]);
        assert_eq!(merge_intervals(&vec![(i32::MIN, 0), (0, i32::MAX)]), [(i32::MIN, i32::MAX)]);
    }
}

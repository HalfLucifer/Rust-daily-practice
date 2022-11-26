/*
   Given inputs are non-overlapped and sorted by start time

   time complexity: O(n)
   space complexity: O(n)
*/
pub fn insert_interval(input: &Vec<(i32, i32)>, interval: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = vec![];
    let mut curr = 0;
    let mut begin = interval.0;
    let mut end = interval.1;

    // Handle inputs ended before interval begins
    while curr < input.len() && input[curr].1 < begin {
        result.push(input[curr]);
        curr += 1;
    }

    // Handle inputs overlapped with interval
    while curr < input.len() && !(input[curr].0 > end) {
        begin = begin.min(input[curr].0);
        end = end.max(input[curr].1);
        curr += 1;
    }
    result.push((begin, end));

    // Handle inputs started after interval ends
    while curr < input.len() {
        result.push(input[curr]);
        curr += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(insert_interval(&vec![(1, 3)], (2, 4)), [(1, 4)]);
        assert_eq!(insert_interval(&vec![(1, 2), (3, 5)], (4, 6)), [(1, 2), (3, 6)]);
        assert_eq!(insert_interval(&vec![(10, 20), (30, 40)], (25, 50)), [(10, 20), (25, 50)]);
        assert_eq!(insert_interval(&vec![(10, 20), (30, 40)], (0, 25)), [(0, 25), (30, 40)]);
        assert_eq!(insert_interval(&vec![(10, 20), (30, 40)], (0, 50)), [(0, 50)]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(insert_interval(&vec![], (0, 0)), [(0, 0)]);
        assert_eq!(insert_interval(&vec![(0, 0)], (0, 0)), [(0, 0)]);
        assert_eq!(insert_interval(&vec![(0, 0), (-1, -1)], (1, 1)), [(0, 0), (-1, -1), (1, 1)]);
        assert_eq!(insert_interval(&vec![(i32::MIN, -1), (1, i32::MAX)], (-1, 1)), [(i32::MIN, i32::MAX)]);
    }

    #[test]
    fn test_nonoverlapped_cases() {
        assert_eq!(insert_interval(&vec![(5, 10), (25, 30)], (15, 20)), [(5, 10), (15, 20), (25, 30)]);
        assert_eq!(insert_interval(&vec![(5, 10), (25, 30)], (35, 40)), [(5, 10), (25, 30), (35, 40)]);
        assert_eq!(insert_interval(&vec![(5, 10), (25, 30)], (-5, 0)), [(-5, 0), (5, 10), (25, 30)]);
    }
}

pub fn next_greater_element(input: &[i32]) -> Vec<Option<i32>> {
    let mut result = vec![None; input.len()];
    let mut q = MonotonicQueue::new();

    // Reversely pushing elements into a monotonic queue
    for i in (0..input.len()).rev() {
        result[i] = q.squash_push(input[i]);
    }

    result
}

struct MonotonicQueue {
    queue: Vec<i32>,
}

impl MonotonicQueue {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn squash_push(&mut self, value: i32) -> Option<i32> {
        // Squash all smaller elements
        while let Some(e) = self.queue.last() {
            if *e <= value {
                self.queue.pop();
            } else {
                break;
            }
        }

        // Get result before pushing value
        let result = self.queue.last().copied();
        self.queue.push(value);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            next_greater_element(&[1, 2, 3, 4, 5]),
            vec![Some(2), Some(3), Some(4), Some(5), None]
        );

        assert_eq!(
            next_greater_element(&[2, 1, 2, 4, 3]),
            vec![Some(4), Some(2), Some(4), None, None]
        );

        assert_eq!(
            next_greater_element(&[9, 10, 1, 7, 5, 6, 6, 6]),
            vec![Some(10), None, Some(7), None, Some(6), None, None, None]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(next_greater_element(&[]), vec![]);
        assert_eq!(next_greater_element(&[0]), vec![None]);
        assert_eq!(next_greater_element(&[0, 0, 0]), vec![None, None, None]);

        assert_eq!(
            next_greater_element(&[i32::MIN, 0, i32::MAX]),
            vec![Some(0), Some(i32::MAX), None]
        );
    }

    #[test]
    fn test_sequential_cases() {
        let expected = [None; 99];
        assert_eq!(
            next_greater_element(&(1..100).rev().collect::<Vec<_>>()),
            expected
        );

        let mut expected = (2..100).map(|i| Some(i)).collect::<Vec<_>>();
        expected.push(None);
        assert_eq!(
            next_greater_element(&(1..100).collect::<Vec<_>>()),
            expected
        );
    }
}

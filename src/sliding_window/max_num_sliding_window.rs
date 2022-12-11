/*
   Find max numbers for each sliding window of a given array

   Method 1: Use monotonic queue
   - Get max number: O(1)
   - Get the desired output: O(n)

   Method 2: Use slice::window iterator
*/
pub fn max_num_sliding_window(arr: &[i32], size: usize) -> Vec<i32> {
    assert!(!arr.is_empty());
    assert!(size > 0);

    if size > arr.len() {
        return vec![];
    }

    let mut result = vec![];
    let mut window = MonotonicQueue::new();

    for i in 0..size - 1 {
        window.push(arr[i]);
    }

    for i in size - 1..arr.len() {
        // Push into monotonic queue
        window.push(arr[i]);

        // The first element in queue is the max number
        if let Some(max) = window.max() {
            result.push(max);
        }

        // Remove out-of-window element
        window.pop(arr[i + 1 - size]);
    }

    result
}

pub fn max_num_sliding_window_by_iterator(arr: &[i32], size: usize) -> Vec<i32> {
    arr.windows(size)
        .map(|w| w.iter().max().unwrap().clone())
        .collect::<Vec<_>>()
}

struct MonotonicQueue {
    queue: Vec<i32>,
}

impl MonotonicQueue {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn push(&mut self, value: i32) {
        while let Some(e) = self.queue.last() {
            if *e < value {
                self.queue.pop();
            } else {
                break;
            }
        }

        self.queue.push(value);
    }

    pub fn pop(&mut self, value: i32) {
        if let Some(e) = self.queue.first() {
            // Remove the head only if it equals to value
            if *e == value {
                self.queue.remove(0);
            }
        }
    }

    pub fn max(&self) -> Option<i32> {
        self.queue.first().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let arr = (0..10).collect::<Vec<_>>();
        for size in 2..10 {
            let result = max_num_sliding_window(&arr, size);
            let expected = max_num_sliding_window_by_iterator(&arr, size);
            assert_eq!(result, expected);
        }

        let arr = (100..200).rev().collect::<Vec<_>>();
        for size in 2..10 {
            let result = max_num_sliding_window(&arr, size);
            let expected = max_num_sliding_window_by_iterator(&arr, size);
            assert_eq!(result, expected);
        }

        let arr = [1, 3, -1, -3, 5, 3, 6, 7];
        for size in 1..=5 {
            let result = max_num_sliding_window(&arr, size);
            let expected = max_num_sliding_window_by_iterator(&arr, size);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(max_num_sliding_window(&[0], 1), vec![0]);
        assert_eq!(max_num_sliding_window(&[0], 99), vec![]);
        assert_eq!(max_num_sliding_window(&[0, 1], 1), vec![0, 1]);
        assert_eq!(max_num_sliding_window(&[0, 1], 2), vec![1]);

        assert_eq!(
            max_num_sliding_window(&[i32::MIN, 0, i32::MAX], 2),
            vec![0, i32::MAX]
        );

        assert_eq!(
            max_num_sliding_window(&[i32::MIN, 0, i32::MAX], 3),
            vec![i32::MAX]
        );
    }

    #[test]
    fn test_same_max_cases() {
        assert_eq!(
            max_num_sliding_window(&[0, 1, 0, 1, 0, 1, 0, 1], 2),
            [1, 1, 1, 1, 1, 1, 1]
        );

        assert_eq!(
            max_num_sliding_window(&[-1, -10, -2, -1, -5, -3, -1, -6], 3),
            [-1, -1, -1, -1, -1, -1]
        );

        assert_eq!(
            max_num_sliding_window(&[-7, -8, 7, 5, 7, 1, 6, 0], 4),
            [7, 7, 7, 7, 7]
        );
    }
}

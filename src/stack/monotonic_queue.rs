pub struct MonotonicQueue {
    queue: Vec<i32>,
}

impl MonotonicQueue {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn push(&mut self, value: i32) {
        while let Some(e) = self.queue.last() {
            // Monotonic descending
            if *e <= value {
                self.queue.pop();
            } else {
                break;
            }
        }

        self.queue.push(value);
    }

    pub fn pop(&mut self, value: i32) {
        if let Some(e) = self.queue.first() {
            // Remove the front element only if it equals to given value
            if *e == value {
                self.queue.remove(0);
            }
        }
    }

    pub fn first(&self) -> Option<i32> {
        self.queue.first().copied()
    }

    pub fn last(&self) -> Option<i32> {
        self.queue.last().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn traverse(&self) -> Vec<i32> {
        self.queue.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut q = MonotonicQueue::new();
        assert!(q.is_empty());
        q.push(3);
        q.push(2);
        q.push(1);
        q.push(0);
        q.push(-1);
        q.push(1);
        assert!(!q.is_empty());
        assert_eq!(q.traverse(), [3, 2, 1]);

        let mut q = MonotonicQueue::new();
        q.push(6);
        q.push(5);
        q.push(4);
        q.push(6);
        q.push(1);
        q.push(2);
        q.push(6);
        assert_eq!(q.first(), Some(6));
        assert_eq!(q.last(), Some(6));
        assert_eq!(q.traverse(), [6]);
    }

    #[test]
    fn test_pop_cases() {
        let mut q = MonotonicQueue::new();
        q.push(3);
        q.push(2);
        q.push(1);
        q.pop(1);
        q.pop(2);
        assert_eq!(q.traverse(), [3, 2, 1]);

        q.pop(3);
        assert_eq!(q.traverse(), [2, 1]);

        q.pop(2);
        assert_eq!(q.traverse(), [1]);

        q.pop(1);
        assert_eq!(q.traverse(), []);

        q.pop(0);
        assert_eq!(q.traverse(), []);
    }

    #[test]
    fn test_edge_cases() {
        let q = MonotonicQueue::new();
        assert_eq!(q.first(), None);
        assert_eq!(q.last(), None);
        assert_eq!(q.traverse(), []);

        let mut q = MonotonicQueue::new();
        for _ in 1..100 {
            q.push(-1);
        }
        assert_eq!(q.traverse(), vec![-1]);

        let mut q = MonotonicQueue::new();
        for i in 1..100 {
            q.push(i);
        }
        assert_eq!(q.traverse(), [99]);

        let mut q = MonotonicQueue::new();
        for i in (1..100).rev() {
            q.push(i);
        }
        assert_eq!(q.traverse(), (1..100).rev().collect::<Vec<_>>());
    }
}

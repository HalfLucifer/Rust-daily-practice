pub struct HitCounter {
    queue: Vec<usize>,
}

impl HitCounter {
    const THESHOLD: usize = 300;

    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    // Store the timestamp of each hit
    pub fn hit(&mut self, timestamp: usize) {
        self.queue.push(timestamp);
    }

    // Get the hit count in the past 5 minutes
    pub fn get(&self, timestamp: usize) -> usize {
        if timestamp <= Self::THESHOLD {
            return self.queue.len();
        }

        for i in 0..self.queue.len() {
            if self.queue[i] > timestamp - Self::THESHOLD {
                return (&self.queue[i..]).len();
            }
        }

        0
    }
}

// For handling a large number of hits per timestamp
pub struct HitCounterEx {
    times: Vec<usize>,
    hits: Vec<usize>,
}

impl HitCounterEx {
    const THESHOLD: usize = 300;

    pub fn new() -> Self {
        Self {
            times: vec![0; Self::THESHOLD],
            hits: vec![0; Self::THESHOLD],
        }
    }

    pub fn hit(&mut self, timestamp: usize, count: usize) {
        let index = timestamp as usize % Self::THESHOLD;

        if self.times[index] != timestamp {
            self.times[index] = timestamp;
            self.hits[index] = count;
        } else {
            self.hits[index] += count;
        }
    }

    pub fn get(&self, timestamp: usize) -> usize {
        let mut res = 0;

        for i in 0..Self::THESHOLD {
            if timestamp >= self.times[i] && timestamp - self.times[i] < Self::THESHOLD {
                res += self.hits[i];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut hc = HitCounter::new();

        assert_eq!(hc.get(0), 0);
        hc.hit(1);
        hc.hit(2);
        hc.hit(3);
        assert_eq!(hc.get(3), 3);
        assert_eq!(hc.get(4), 3);
        assert_eq!(hc.get(100), 3);

        hc.hit(300);
        assert_eq!(hc.get(300), 4);
        assert_eq!(hc.get(301), 3);

        hc.hit(301);
        assert_eq!(hc.get(301), 4);
        assert_eq!(hc.get(302), 3);
        assert_eq!(hc.get(303), 2);

        assert_eq!(hc.get(600), 1);
        assert_eq!(hc.get(601), 0);
        assert_eq!(hc.get(usize::MAX), 0);
    }

    #[test]
    fn test_large_hit_cases() {
        let mut hc = HitCounterEx::new();
        assert_eq!(hc.get(0), 0);

        hc.hit(1, 10000);
        hc.hit(50, 10000);
        hc.hit(300, 10000);

        assert_eq!(hc.get(1), 10000);
        assert_eq!(hc.get(99), 20000);
        assert_eq!(hc.get(300), 30000);
        assert_eq!(hc.get(301), 20000);
        assert_eq!(hc.get(350), 10000);

        hc.hit(351, 1);
        assert_eq!(hc.get(351), 10001);
        assert_eq!(hc.get(650), 1);
        assert_eq!(hc.get(651), 0);
    }
}

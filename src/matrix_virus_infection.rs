/*
   Virus will spread infection to neighborhood nodes each day

   Goal: determine minimum days to infect all healthy nodes
   - 0: empty
   - 1: healthy node
   - 2: infected node
*/
pub struct Virus {
    matrix: Vec<Vec<i32>>,
}

impl Virus {
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    const HEALTHY: i32 = 1;
    const INFECTED: i32 = 2;

    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix: matrix }
    }

    pub fn infect(&self) -> Option<usize> {
        let mut m = self.matrix.clone();
        let mut queue = vec![];
        let mut healthy_node = 0;
        let mut days_to_infect_all = 1;

        for i in 0..m.len() {
            for j in 0..m[0].len() {
                if m[i][j] == Self::INFECTED {
                    queue.push((i, j));
                } else if m[i][j] == Self::HEALTHY {
                    healthy_node += 1;
                }
            }
        }

        let mut turns = queue.len();
        if turns == 0 {
            return None;
        }

        while !queue.is_empty() && healthy_node > 0 {
            let virus = queue.remove(0);
            let x = virus.0;
            let y = virus.1;

            Self::DIR.iter().for_each(|d| {
                let i = x as i32 + d.0;
                let j = y as i32 + d.1;

                if i > -1
                    && i < m.len() as i32
                    && j > -1
                    && j < m[0].len() as i32
                    && m[i as usize][j as usize] == Self::HEALTHY
                {
                    m[i as usize][j as usize] = Self::INFECTED;
                    queue.push((i as usize, j as usize));
                    healthy_node -= 1;
                }
            });

            turns -= 1;

            if turns == 0 {
                turns = queue.len();

                if healthy_node > 0 {
                    days_to_infect_all += 1;
                }
            }
        }

        if healthy_node > 0 {
            return None;
        }

        Some(days_to_infect_all)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let v = Virus::new(vec![vec![2, 0], vec![0, 1]]);
        assert_eq!(v.infect(), None);

        let v = Virus::new(vec![vec![2, 1], vec![1, 0]]);
        assert_eq!(v.infect(), Some(1));

        let v = Virus::new(vec![vec![2, 1], vec![1, 1]]);
        assert_eq!(v.infect(), Some(2));

        let v = Virus::new(vec![vec![1, 1, 1], vec![1, 2, 1], vec![1, 1, 1]]);
        assert_eq!(v.infect(), Some(2));

        let v = Virus::new(vec![vec![1, 1, 1], vec![1, 2, 1], vec![1, 1, 1]]);
        assert_eq!(v.infect(), Some(2));

        let v = Virus::new(vec![
            vec![1, 1, 1, 2],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ]);
        assert_eq!(v.infect(), Some(6));

        let v = Virus::new(vec![
            vec![1, 1, 1, 2],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![2, 1, 1, 1],
        ]);
        assert_eq!(v.infect(), Some(3));

        let v = Virus::new(vec![
            vec![2, 1, 1, 2],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![2, 1, 1, 2],
        ]);
        assert_eq!(v.infect(), Some(2));
    }

    #[test]
    fn test_edge_cases() {
        let v = Virus::new(vec![]);
        assert_eq!(v.infect(), None);

        let v = Virus::new(vec![vec![0]]);
        assert_eq!(v.infect(), None);

        let v = Virus::new(vec![vec![1]]);
        assert_eq!(v.infect(), None);

        let v = Virus::new(vec![vec![2]]);
        assert_eq!(v.infect(), Some(1));

        let v = Virus::new(vec![vec![2, 1]]);
        assert_eq!(v.infect(), Some(1));

        let v = Virus::new(vec![vec![0, 1, 2]]);
        assert_eq!(v.infect(), Some(1));

        let v = Virus::new(vec![vec![1, 1, 2]]);
        assert_eq!(v.infect(), Some(2));

        let mut m = vec![1; 99];
        m.push(2);
        let v = Virus::new(vec![m]);
        assert_eq!(v.infect(), Some(99));
    }

    #[test]
    fn test_larger_cases() {
        let v = Virus::new(vec![
            vec![2, 1, 1, 1, 2],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0],
            vec![2, 1, 1, 0, 1],
        ]);
        assert_eq!(v.infect(), None);

        let v = Virus::new(vec![
            vec![2, 1, 2, 1, 2],
            vec![1, 1, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 1, 1],
            vec![2, 1, 2, 1, 2],
        ]);
        assert_eq!(v.infect(), Some(1));

        let v = Virus::new(vec![
            vec![2, 1, 1, 1, 2],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![2, 1, 1, 1, 2],
        ]);
        assert_eq!(v.infect(), Some(4));

        let v = Virus::new(vec![
            vec![2, 1, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 2, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
        ]);
        assert_eq!(v.infect(), Some(4));

        let v = Virus::new(vec![
            vec![2, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ]);
        assert_eq!(v.infect(), Some(16));
    }
}

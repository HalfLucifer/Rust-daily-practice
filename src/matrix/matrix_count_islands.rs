pub struct Islands {
    matrix: Vec<Vec<i32>>,
}

impl Islands {
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix: matrix }
    }

    pub fn count_islands(&self) -> usize {
        let mut m = self.matrix.clone();
        let mut count = 0;

        for i in 0..m.len() {
            for j in 0..m[0].len() {
                if m[i][j] == 1 {
                    count += 1;
                    self.flip_board(&mut m, i as i32, j as i32);
                }
            }
        }

        count
    }

    fn flip_board(&self, matrix: &mut Vec<Vec<i32>>, row: i32, col: i32) {
        let mut queue = vec![(row, col)];

        while !queue.is_empty() {
            let pos = queue.remove(0);
            let x = pos.0;
            let y = pos.1;

            if x < 0
                || y < 0
                || x > matrix.len() as i32 - 1
                || y > matrix[0].len() as i32 - 1
                || matrix[x as usize][y as usize] == 0 {
                continue;
            }

            matrix[x as usize][y as usize] = 0;

            Islands::DIR.iter().for_each(|d| {
                queue.push((x + d.0, y + d.1));
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let i = Islands::new(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]);
        assert_eq!(i.count_islands(), 0);

        let i = Islands::new(vec![
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
        ]);
        assert_eq!(i.count_islands(), 1);

        let i = Islands::new(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]);
        assert_eq!(i.count_islands(), 5);

        let i = Islands::new(vec![
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 1],
        ]);
        assert_eq!(i.count_islands(), 1);

        let i = Islands::new(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0],
            vec![1, 1, 1, 0, 1],
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 1, 1, 1],
        ]);
        assert_eq!(i.count_islands(), 2);

        let i = Islands::new(vec![
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
        ]);
        assert_eq!(i.count_islands(), 13);
    }

    #[test]
    fn test_edge_cases() {
        let i = Islands::new(vec![]);
        assert_eq!(i.count_islands(), 0);

        let i = Islands::new(vec![vec![0]]);
        assert_eq!(i.count_islands(), 0);

        let i = Islands::new(vec![vec![1]]);
        assert_eq!(i.count_islands(), 1);

        let i = Islands::new(vec![vec![1, 0, 1]]);
        assert_eq!(i.count_islands(), 2);
    }
}

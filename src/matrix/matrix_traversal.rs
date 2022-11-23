use std::collections::HashSet;

pub struct Matrix {
    matrix: Vec<Vec<i32>>,
}

impl Matrix {
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix: matrix }
    }

    pub fn depth_first_traverse(&self, origin_column: i32, origin_row: i32) -> Vec<i32> {
        fn depth_first_traverse_recursive(
            matrix: &Vec<Vec<i32>>,
            row: i32,
            column: i32,
            visited: &mut HashSet<(i32, i32)>,
            result: &mut Vec<i32>,
        ) {
            if row < 0
                || column < 0
                || row > matrix.len() as i32 - 1
                || column > matrix[0].len() as i32 - 1
                || visited.contains(&(row, column)) {
                return;
            }

            visited.insert((row, column));
            result.push(matrix[row as usize][column as usize]);

            Matrix::DIR.iter().for_each(|d| {
                depth_first_traverse_recursive(matrix, row + d.0, column + d.1, visited, result);
            });
        }

        let mut result = vec![];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        depth_first_traverse_recursive(&self.matrix, origin_row, origin_column, &mut visited, &mut result);

        result
    }

    pub fn breadth_first_traverse(&self, origin_column: i32, origin_row: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut queue = vec![(origin_row, origin_column)];

        while !queue.is_empty() {
            let pos = queue.remove(0);
            let row = pos.0;
            let column = pos.1;

            if row < 0
                || column < 0
                || row > self.matrix.len() as i32 - 1
                || column > self.matrix[0].len() as i32 - 1
                || visited.contains(&(row, column)) {
                continue;
            }

            visited.insert((row, column));
            result.push(self.matrix[row as usize][column as usize]);

            Matrix::DIR.iter().for_each(|d| {
                queue.push((row + d.0, column + d.1));
            });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_cases() {
        let m = Matrix::new(vec![
            vec![0,  1,  2,  3,  4 ],
            vec![5,  6,  7,  8,  9 ],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
        ]);

        assert_eq!(m.depth_first_traverse(0, 0), [0, 1, 2, 3, 4, 9, 14, 19, 18, 13, 8, 7, 12, 17, 16, 11, 6, 5, 10, 15]);
        assert_eq!(m.depth_first_traverse(4, 0), [4, 9, 14, 19, 18, 13, 8, 3, 2, 7, 12, 17, 16, 11, 6, 1, 0, 5, 10, 15]);
        assert_eq!(m.depth_first_traverse(0, 3), [15, 10, 5, 0, 1, 2, 3, 4, 9, 14, 19, 18, 13, 8, 7, 12, 17, 16, 11, 6]);
        assert_eq!(m.depth_first_traverse(4, 3), [19, 14, 9, 4, 3, 8, 13, 18, 17, 12, 7, 2, 1, 6, 11, 16, 15, 10, 5, 0]);
        assert_eq!(m.depth_first_traverse(2, 2), [12, 7, 2, 3, 4, 9, 14, 19, 18, 13, 8, 17, 16, 11, 6, 1, 0, 5, 10, 15]);
    }

    #[test]
    fn test_bfs_cases() {
        let m = Matrix::new(vec![
            vec![0,  1,  2,  3,  4 ],
            vec![5,  6,  7,  8,  9 ],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
        ]);

        assert_eq!(m.breadth_first_traverse(0, 0), [0, 1, 5, 2, 6, 10, 3, 7, 11, 15, 4, 8, 12, 16, 9, 13, 17, 14, 18, 19]);
        assert_eq!(m.breadth_first_traverse(4, 0), [4, 9, 3, 14, 8, 2, 19, 13, 7, 1, 18, 12, 6, 0, 17, 11, 5, 16, 10, 15]);
        assert_eq!(m.breadth_first_traverse(0, 3), [15, 10, 16, 5, 11, 17, 0, 6, 12, 18, 1, 7, 13, 19, 2, 8, 14, 3, 9, 4]);
        assert_eq!(m.breadth_first_traverse(4, 3), [19, 14, 18, 9, 13, 17, 4, 8, 12, 16, 3, 7, 11, 15, 2, 6, 10, 1, 5, 0]);
        assert_eq!(m.breadth_first_traverse(2, 2), [12, 7, 13, 17, 11, 2, 8, 6, 14, 18, 16, 10, 3, 1, 9, 5, 19, 15, 4, 0]);
    }

    #[test]
    fn test_edge_cases() {
        let m = Matrix::new(vec![]);
        assert_eq!(m.depth_first_traverse(0, 0), []);
        assert_eq!(m.breadth_first_traverse(0, 0), []);

        let m = Matrix::new(vec![vec![0]]);
        assert_eq!(m.depth_first_traverse(0, 0), [0]);
        assert_eq!(m.breadth_first_traverse(0, 0), [0]);
    }
}

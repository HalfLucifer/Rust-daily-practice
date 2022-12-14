pub struct Matrix {
    matrix: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix: matrix }
    }

    pub fn get_matrix(&self) -> Vec<Vec<i32>> {
        self.matrix.clone()
    }

    pub fn rotate_right(&mut self) {
        if self.matrix.is_empty() || self.matrix[0].is_empty() {
            return;
        }

        let n = self.matrix.len();

        for layer in 0..n / 2 {
            let first = layer;
            let last = n - 1 - layer;

            for i in first..last {
                let offset = i - first; // offset=[0..last-first]
                let top = self.matrix[first][i]; // save top left for later

                self.matrix[first][i] = self.matrix[last - offset][first]; // top left <- bottom left
                self.matrix[last - offset][first] = self.matrix[last][last - offset]; // bottom left <- bottom right
                self.matrix[last][last - offset] = self.matrix[i][last]; // bottom right <- top right
                self.matrix[i][last] = top; // top right <- top left
            }
        }
    }

    pub fn rotate_left(&mut self) {
        if self.matrix.is_empty() || self.matrix[0].is_empty() {
            return;
        }

        let n = self.matrix.len();

        for layer in 0..n / 2 {
            let first = layer;
            let last = n - 1 - layer;

            for i in first..last {
                let offset = i - first; // offset=[0..last-first]
                let top = self.matrix[first][i]; // save top left for later

                self.matrix[first][i] = self.matrix[i][last]; // top left <- top right
                self.matrix[i][last] = self.matrix[last][last - offset]; // top right <- bottom right
                self.matrix[last][last - offset] = self.matrix[last - offset][first]; // bottom right <- bottom left
                self.matrix[last - offset][first] = top; // bottom left <- top left
            }
        }
    }

    pub fn rotate_right_by_iterator(&mut self) -> Vec<Vec<i32>> {
        (0..self.matrix.len())
            .map(|col| self.matrix.iter().rev().map(|row| row[col]).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
        m.rotate_right();
        assert_eq!(m.get_matrix(), [[2, 0], [3, 1]]);

        let mut m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
        m.rotate_left();
        assert_eq!(m.get_matrix(), [[1, 3], [0, 2]]);

        let mut m = Matrix::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        m.rotate_right();
        assert_eq!(m.get_matrix(), [[6, 3, 0], [7, 4, 1], [8, 5, 2]]);

        let mut m = Matrix::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        m.rotate_left();
        assert_eq!(m.get_matrix(), [[2, 5, 8], [1, 4, 7], [0, 3, 6]]);
    }

    #[test]
    fn test_edge_cases() {
        let mut m = Matrix::new(vec![]);
        m.rotate_right();
        m.rotate_left();

        let mut m = Matrix::new(vec![vec![0]]);
        m.rotate_right();
        assert_eq!(m.get_matrix(), [[0]]);
        m.rotate_left();
        assert_eq!(m.get_matrix(), [[0]]);

        let mut m = Matrix::new(vec![vec![0, i32::MAX], vec![i32::MIN, 0]]);
        m.rotate_right();
        m.rotate_left();
        assert_eq!(m.get_matrix(), [[0, i32::MAX], [i32::MIN, 0]]);
    }

    #[test]
    fn test_rotate_by_iterator_cases() {
        let mut m = Matrix::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        let expected = m.rotate_right_by_iterator();
        m.rotate_right();
        assert_eq!(m.get_matrix(), expected);

        let mut m = Matrix::new(vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
        ]);
        let expected = m.rotate_right_by_iterator();
        m.rotate_right();
        assert_eq!(m.get_matrix(), expected);

        let mut m = Matrix::new(vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
            vec![20, 21, 22, 23, 24],
        ]);
        let expected = m.rotate_right_by_iterator();
        m.rotate_right();
        assert_eq!(m.get_matrix(), expected);
    }

    #[test]
    fn test_larger_cases() {
        let mut m = Matrix::new(vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
            vec![20, 21, 22, 23, 24],
        ]);

        m.rotate_left();
        assert_eq!(
            m.get_matrix(),
            [
                [4, 9, 14, 19, 24],
                [3, 8, 13, 18, 23],
                [2, 7, 12, 17, 22],
                [1, 6, 11, 16, 21],
                [0, 5, 10, 15, 20]
            ]
        );

        m.rotate_left();
        m.rotate_left();
        m.rotate_left();
        assert_eq!(
            m.get_matrix(),
            [
                [0, 1, 2, 3, 4],
                [5, 6, 7, 8, 9],
                [10, 11, 12, 13, 14],
                [15, 16, 17, 18, 19],
                [20, 21, 22, 23, 24]
            ]
        );

        m.rotate_right();
        assert_eq!(
            m.get_matrix(),
            [
                [20, 15, 10, 5, 0],
                [21, 16, 11, 6, 1],
                [22, 17, 12, 7, 2],
                [23, 18, 13, 8, 3],
                [24, 19, 14, 9, 4]
            ]
        );

        m.rotate_right();
        m.rotate_right();
        m.rotate_right();
        assert_eq!(
            m.get_matrix(),
            [
                [0, 1, 2, 3, 4],
                [5, 6, 7, 8, 9],
                [10, 11, 12, 13, 14],
                [15, 16, 17, 18, 19],
                [20, 21, 22, 23, 24]
            ]
        );
    }
}

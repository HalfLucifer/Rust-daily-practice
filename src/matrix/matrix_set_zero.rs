/*
    Set entire row and column to zero for zero element

    Time complexity: O(m*n)
    Space complexity: O(m+n)
*/
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut row_zeros = vec![false; matrix.len()];
    let mut col_zeros = vec![false; matrix[0].len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                row_zeros[i] = true;
                col_zeros[j] = true;
            }
        }
    }

    for i in 0..matrix.len() {
        if row_zeros[i] {
            for j in 0..matrix[0].len() {
                matrix[i][j] = 0;
            }
        }
    }

    for j in 0..matrix[0].len() {
        if col_zeros[j] {
            for i in 0..matrix.len() {
                matrix[i][j] = 0;
            }
        }
    }
}

/*
    Time complexity: O(m*n)
    Space complexity: O(1)
*/
pub fn set_zeroes_optimized(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut first_row_has_zero = false;
    let mut first_col_has_zero = false;

    // Check whether 1st row has any zero
    for i in 0..m {
        if matrix[i][0] == 0 {
            first_row_has_zero = true;
        }
    }

    // Check whether 1st column has any zero
    for j in 0..n {
        if matrix[0][j] == 0 {
            first_col_has_zero = true;
        }
    }

    // Store zeros at the 1st row & 1st column
    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    // Kill [1..m] rows containing zero
    for i in 1..m {
        if matrix[i][0] == 0 {
            for j in 0..n {
                matrix[i][j] = 0;
            }
        }
    }

    // Kill [1..n] columns containing zero
    for j in 1..n {
        if matrix[0][j] == 0 {
            for i in 0..m {
                matrix[i][j] = 0;
            }
        }
    }

    // Kill 1st row if necessary
    if first_row_has_zero {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }

    // Kill 1st column if necessary
    if first_col_has_zero {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut mat = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let mut mat_clone = mat.clone();

        set_zeroes(&mut mat);
        set_zeroes_optimized(&mut mat_clone);

        assert_eq!(mat, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
        assert_eq!(mat_clone, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut mat = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let mut mat_clone = mat.clone();

        set_zeroes(&mut mat);
        set_zeroes_optimized(&mut mat_clone);

        assert_eq!(mat, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
        assert_eq!(mat_clone, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
    }

    #[test]
    fn test_edge_cases() {
        let mut mat1 = vec![vec![0]];
        let mut mat2 = vec![vec![0, 1], vec![1, 1]];
        let mut mat3 = vec![vec![1, 1], vec![1, 1]];

        set_zeroes_optimized(&mut mat1);
        set_zeroes_optimized(&mut mat2);
        set_zeroes_optimized(&mut mat3);

        assert_eq!(mat1, vec![vec![0]]);
        assert_eq!(mat2, vec![vec![0, 0], vec![0, 1]]);
        assert_eq!(mat3, vec![vec![1, 1], vec![1, 1]]);
    }
}

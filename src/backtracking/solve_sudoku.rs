/*
    Write a program to solve a Sudoku puzzle by filling the empty cells.

    A sudoku solution must satisfy all of the following rules:
    - Each of the digits 1-9 must occur exactly once in each row.
    - Each of the digits 1-9 must occur exactly once in each column.
    - Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes
      of the grid.
      
    The '.' character indicates empty cells.
*/
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    backtrack(board, 0, 0);
}

fn is_valid(board: &Vec<Vec<char>>, r: usize, c: usize, num: char) -> bool {
    for i in 0..9 {
        // Check the same row & column
        if board[i][c] == num || board[r][i] == num {
            return false;
        }

        // Check 3x3 sub-box
        let row = (r / 3) * 3 + (i / 3); // start row + [0..3] index
        let col = (c / 3) * 3 + (i % 3); // start column + [0..3] index

        if board[row][col] == num {
            return false;
        }
    }

    true
}

fn backtrack(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // Found a final solution
    if row == 9 {
        return true;
    }

    // All columns are proceeded, go to next row
    if col == 9 {
        return backtrack(board, row + 1, 0);
    }

    // Skip existed element, go to next column
    if board[row][col] != '.' {
        return backtrack(board, row, col + 1);
    }

    for c in '1'..='9' {
        if is_valid(board, row, col, c) {
            board[row][col] = c;

            if backtrack(board, row, col + 1) {
                // Found a valid number
                return true;
            }

            board[row][col] = '.';
        }
    }

    // All numbers for current element are invalid, go backtracking
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        solve_sudoku(&mut board);

        assert_eq!(
            board, vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ]);
    }

    #[test]
    fn test_all_empty_cases() {
        let mut board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        solve_sudoku(&mut board);

        assert_eq!(
            board, vec![
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'],
            vec!['4', '5', '6', '7', '8', '9', '1', '2', '3'],
            vec!['7', '8', '9', '1', '2', '3', '4', '5', '6'],
            vec!['2', '1', '4', '3', '6', '5', '8', '9', '7'],
            vec!['3', '6', '5', '8', '9', '7', '2', '1', '4'],
            vec!['8', '9', '7', '2', '1', '4', '3', '6', '5'],
            vec!['5', '3', '1', '6', '4', '2', '9', '7', '8'],
            vec!['6', '4', '2', '9', '7', '8', '5', '3', '1'],
            vec!['9', '7', '8', '5', '3', '1', '6', '4', '2'],
        ]);
    }

    #[test]
    fn test_edge_cases() {
        let mut board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        solve_sudoku(&mut board);

        assert_eq!(
            board, vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
    }
}

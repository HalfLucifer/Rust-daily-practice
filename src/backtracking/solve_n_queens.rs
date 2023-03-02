/*
    The n-queens puzzle is the problem of placing n queens on an n x n chessboard such
    that no two queens attack each other. Given an integer n, return all distinct
    solutions to the n-queens puzzle.

    Each solution contains a distinct board configuration of the n-queens' placement,
    where 'Q' and '.' both indicate a queen and an empty space, respectively.
*/
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn backtrack(board: &mut Vec<Vec<char>>, result: &mut Vec<Vec<String>>, row: usize) {
        if row == board.len() {
            result.push(
                board
                    .iter()
                    .map(|row| row.iter().collect::<String>())
                    .collect::<Vec<String>>(),
            );
            return;
        }

        for j in 0..board.len() {
            if is_valid(board, row, j) {
                board[row][j] = 'Q';
                backtrack(board, result, row + 1);
                board[row][j] = '.';
            }
        }
    }

    let mut board = vec![vec!['.'.to_owned(); n as usize]; n as usize];
    let mut result = vec![];
    backtrack(&mut board, &mut result, 0);
    result
}

pub fn total_n_queens(n: i32) -> i32 {
    fn backtrack(board: &mut Vec<Vec<char>>, result: &mut i32, row: usize) {
        if row == board.len() {
            *result += 1;
            return;
        }

        for j in 0..board.len() {
            if is_valid(board, row, j) {
                board[row][j] = 'Q';
                backtrack(board, result, row + 1);
                board[row][j] = '.';
            }
        }
    }

    let mut board = vec![vec!['.'.to_owned(); n as usize]; n as usize];
    let mut result = 0;
    backtrack(&mut board, &mut result, 0);
    result
}

fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // Check upper direction
    for i in 0..row {
        if board[i][col] == 'Q' {
            return false;
        }
    }

    // Check upper-left direction
    let mut i = row as isize - 1;
    let mut j = col as isize - 1;
    while i >= 0 && j >= 0 {
        if board[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    // Check upper-right direction
    let mut i = row as isize - 1;
    let mut j = col as isize + 1;
    while i >= 0 && j < board.len() as isize {
        if board[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_cases() {
        let empty: Vec<Vec<String>> = vec![];
        assert_eq!(solve_n_queens(1), vec![vec!["Q"]]);
        assert_eq!(solve_n_queens(2), empty);
        assert_eq!(solve_n_queens(3), empty);

        assert_eq!(
            solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );

        assert_eq!(
            solve_n_queens(5),
            vec![
                vec!["Q....", "..Q..", "....Q", ".Q...", "...Q."],
                vec!["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
                vec![".Q...", "...Q.", "Q....", "..Q..", "....Q"],
                vec![".Q...", "....Q", "..Q..", "Q....", "...Q."],
                vec!["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
                vec!["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
                vec!["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
                vec!["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
                vec!["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
                vec!["....Q", "..Q..", "Q....", "...Q.", ".Q..."]
            ]
        );

        assert_eq!(
            solve_n_queens(6),
            vec![
                vec![".Q....", "...Q..", ".....Q", "Q.....", "..Q...", "....Q."],
                vec!["..Q...", ".....Q", ".Q....", "....Q.", "Q.....", "...Q.."],
                vec!["...Q..", "Q.....", "....Q.", ".Q....", ".....Q", "..Q..."],
                vec!["....Q.", "..Q...", "Q.....", ".....Q", "...Q..", ".Q...."]
            ]
        );
    }

    #[test]
    fn test_count_cases() {
        assert_eq!(total_n_queens(1), 1);
        assert_eq!(total_n_queens(2), 0);
        assert_eq!(total_n_queens(3), 0);
        assert_eq!(total_n_queens(4), 2);
        assert_eq!(total_n_queens(5), 10);
        assert_eq!(total_n_queens(6), 4);
        assert_eq!(total_n_queens(7), 40);
        assert_eq!(total_n_queens(8), 92);
        assert_eq!(total_n_queens(9), 352);
    }
}

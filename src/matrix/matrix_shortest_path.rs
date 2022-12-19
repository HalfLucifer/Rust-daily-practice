/*
    Shortest Path
      * is the start
      # is a goal (there are multiple)
      O is free space
      X is a wall
*/
use std::collections::VecDeque;

pub fn matrix_shortest_path(mat: Vec<Vec<char>>) -> i32 {
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let m = mat.len();
    let n = mat[0].len();
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; n]; m];
    let mut steps = 0;

    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == '*' {
                queue.push_back((i, j));
                visited[i][j] = true;
                break;
            }
        }
    }

    while !queue.is_empty() {
        let count = queue.len();

        for _ in 0..count {
            let (r, c) = queue.pop_front().unwrap();

            if mat[r as usize][c as usize] == '#' {
                return steps;
            }

            for d in DIR {
                let row = r as i32 + d.0;
                let col = c as i32 + d.1;

                if row >= 0
                    && row < m as i32
                    && col >= 0
                    && col < n as i32
                    && !visited[row as usize][col as usize]
                    && mat[row as usize][col as usize] != 'X'
                {
                    visited[row as usize][col as usize] = true;
                    queue.push_back((row as usize, col as usize));
                }
            }
        }

        steps += 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            matrix_shortest_path(vec![vec!['O', '*'], vec!['#', 'O']]),
            2
        );

        assert_eq!(
            matrix_shortest_path(vec![
                vec!['X', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', '*', 'O', 'O', 'O', 'X'],
                vec!['X', 'O', 'O', '#', 'O', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'X']
            ]),
            3
        );

        assert_eq!(
            matrix_shortest_path(vec![
                vec!['X', 'X', 'X', 'X', 'X'],
                vec!['X', '*', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', '#', 'X'],
                vec!['X', 'X', 'X', 'X', 'X']
            ]),
            -1
        );

        assert_eq!(
            matrix_shortest_path(vec![
                vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', '*', 'O', 'X', 'O', '#', 'O', 'X'],
                vec!['X', 'O', 'O', 'X', 'O', 'O', 'X', 'X'],
                vec!['X', 'O', 'O', 'O', 'O', '#', 'O', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X']
            ]),
            6
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(matrix_shortest_path(vec![vec!['*']]), -1);
        assert_eq!(matrix_shortest_path(vec![vec!['X', '*'], vec!['#', 'X']]), -1);
        assert_eq!(matrix_shortest_path(vec![vec!['O', 'X']]), -1);
        assert_eq!(matrix_shortest_path(vec![vec!['#', '*']]), 1);
        assert_eq!(matrix_shortest_path(vec![vec!['#'], vec!['*']]), 1);
    }
}

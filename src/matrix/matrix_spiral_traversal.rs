pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let size = matrix.len() * matrix[0].len();
    let mut result = vec![];
    let mut left = 0;
    let mut right = matrix[0].len() - 1;
    let mut top = 0;
    let mut bottom = matrix.len() - 1;
    let mut row = 0;
    let mut col = 0;
    let mut curr_dir = DIR[0];

    while result.len() < size {
        result.push(matrix[row][col]);

        match curr_dir {
            (0, 1) if col == right => {
                top += 1;
                curr_dir = DIR[1];
            }

            (1, 0) if row == bottom => {
                if right == 0 {
                    break;
                }

                right -= 1;
                curr_dir = DIR[2];
            }

            (0, -1) if col == left => {
                if bottom == 0 {
                    break;
                }

                bottom -= 1;
                curr_dir = DIR[3];
            }

            (-1, 0) if row == top => {
                left += 1;
                curr_dir = DIR[0];
            }

            _ => {}
        }

        row = (row as i32 + curr_dir.0) as usize;
        col = (col as i32 + curr_dir.1) as usize;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            spiral_order(vec![vec![1, 2], vec![3, 4],]),
            vec![1, 2, 4, 3,]
        );

        assert_eq!(
            spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );

        assert_eq!(
            spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]),
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(spiral_order(vec![vec![0]]), vec![0]);
        assert_eq!(spiral_order(vec![vec![0, 1]]), vec![0, 1]);
        assert_eq!(spiral_order(vec![vec![0], vec![1]]), vec![0, 1]);
    }
}

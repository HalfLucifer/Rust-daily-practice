pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut result = vec![vec![0; n as usize]; n as usize];
    let mut left = 0;
    let mut right = n as usize - 1;
    let mut top = 0;
    let mut bottom = n as usize - 1;
    let mut row = 0;
    let mut col = 0;
    let mut curr_dir = DIR[0];
    let mut num = 1;

    while num <= n * n {
        result[row][col] = num;

        match curr_dir {
            (0, 1) if col == right => {
                top += 1;
                curr_dir = DIR[1];
            }

            (1, 0) if row == bottom => {
                right -= 1;
                curr_dir = DIR[2];
            }

            (0, -1) if col == left => {
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
        num += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            generate_matrix(2),
            vec![vec![1, 2], vec![4, 3]]);

        assert_eq!(
            generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );

        assert_eq!(
            generate_matrix(4),
            vec![vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(generate_matrix(1), vec![vec![1]]);
    }
}

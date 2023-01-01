pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    fn flood_fill_dfs(
        image: &Vec<Vec<i32>>,
        row: i32,
        col: i32,
        new_color: i32,
        old_color: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        if row < 0 || row > image.len() as i32 - 1 || col < 0 || col > image[0].len() as i32 - 1 {
            return;
        }

        if result[row as usize][col as usize] == new_color {
            return;
        }

        if result[row as usize][col as usize] == old_color {
            result[row as usize][col as usize] = new_color;

            flood_fill_dfs(image, row + 1, col, new_color, old_color, result);
            flood_fill_dfs(image, row - 1, col, new_color, old_color, result);
            flood_fill_dfs(image, row, col + 1, new_color, old_color, result);
            flood_fill_dfs(image, row, col - 1, new_color, old_color, result);
        }
    }

    let mut result: Vec<Vec<i32>> = image.clone();

    if sr >= 0 && sc >= 0 && sr <= image.len() as i32 && sc <= image[0].len() as i32 {
        let old_color = image[sr as usize][sc as usize];

        if new_color != old_color {
            flood_fill_dfs(&image, sr, sc, new_color, old_color, &mut result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 0]], 0, 0, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 0]]
        );

        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 0]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 0]]
        );

        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 0]], 2, 0, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 0]]
        );

        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 0]], 2, 2, 2),
            vec![vec![1, 1, 1], vec![1, 1, 2], vec![1, 2, 2]]
        );

        assert_eq!(
            flood_fill(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![1, 0, 1], vec![0, 2, 0], vec![1, 0, 1]]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(flood_fill(vec![vec![0]], 0, 0, 0), vec![vec![0]]);
        assert_eq!(flood_fill(vec![vec![0]], 0, 0, 1), vec![vec![1]]);
        assert_eq!(flood_fill(vec![vec![0]], 100, 100, 1), vec![vec![0]]);
    }
}

pub fn three_sum(arr: &[i32], target: i32) -> Vec<Vec<i32>> {
    if arr.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut input = arr.to_vec();
    let mut index = 0;

    input.sort();

    while index < input.len() {
        let curr = input[index];
        let sum = target - curr;
        let two_sum = find_all_two_sum(&input[index + 1..], sum);

        two_sum.iter().for_each(|p| {
            result.push(vec![curr, p.0, p.1]);
        });

        while index + 1 < input.len() && input[index] == input[index + 1] {
            index += 1;
        }

        index += 1;
    }

    result
}

fn find_all_two_sum(arr: &[i32], target: i32) -> Vec<(i32, i32)> {
    if arr.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let sum = arr[left] + arr[right];

        if sum > target {
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            let l = arr[left];
            let r = arr[right];

            result.push((l, r));

            while arr[left] == l && left < arr.len() - 1 {
                left += 1;
            }

            while arr[right] == r && right > 0 {
                right -= 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(three_sum(&[-1, 1, 0], 0), [[-1, 0, 1]]);
        assert_eq!(three_sum(&[-2, 1, 0, -1, 2], 0), [[-2, 0, 2], [-1, 0, 1]]);
        assert_eq!(three_sum(&[-9, 7, -5, 3, -1, 0, 2, -4, 6, -8], 0), [[-9, 2, 7], [-9, 3, 6], [-8, 2, 6], [-5, -1, 6], [-5, 2, 3]]);
        assert_eq!(three_sum(&(-10..=0).collect::<Vec<_>>(), -5), [[-4, -1, 0], [-3, -2, 0]]);
        assert_eq!(three_sum(&(0..10).collect::<Vec<_>>(), 10), [[0, 1, 9], [0, 2, 8], [0, 3, 7], [0, 4, 6], [1, 2, 7], [1, 3, 6], [1, 4, 5], [2, 3, 5]]);
    }

    #[test]
    fn test_edge_cases() {
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(&[], 0), expected);
        assert_eq!(three_sum(&[0], 0), expected);
        assert_eq!(three_sum(&[0, 0], 0), expected);
        assert_eq!(three_sum(&[0, 0, 0], 0), [[0, 0, 0]]);
        assert_eq!(three_sum(&[0, i32::MAX, i32::MIN + 1], 0), [[i32::MIN + 1, 0, i32::MAX]]);
    }

    #[test]
    fn test_repeated_input_cases() {
        assert_eq!(three_sum(&[0, 0, 0, 1, 1, 1, -1, -1, -1], 0), [[-1, 0, 1], [0, 0, 0]]);
        assert_eq!(three_sum(&[-2, 1, 0, 0, 0, 1, -1, 2], 0), [[-2, 0, 2], [-2, 1, 1], [-1, 0, 1], [0, 0, 0]]);

        let mut input = vec![];
        for i in 0..6 {
            for _ in 0..i {
                input.push(i);
                input.push(-i);
            }
        }
        assert_eq!(three_sum(&input, 0), [[-5, 1, 4], [-5, 2, 3], [-4, -1, 5], [-4, 1, 3], [-4, 2, 2], [-3, -2, 5], [-3, -1, 4], [-3, 1, 2], [-2, -2, 4], [-2, -1, 3]]);
    }
}

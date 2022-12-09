pub fn rotate_array(arr: &mut Vec<i32>, step: usize) {
    if arr.len() <= 1 {
        return;
    }

    let step = step % arr.len();
    let target = arr.len() - step;

    // Reverse 1st portion
    arr[..target].reverse();

    // Reverse 2nd portion
    arr[target..].reverse();

    // Reverse whole array
    arr.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut arr = vec![1, 2, 3, 4, 5];
        rotate_array(&mut arr, 1);
        assert_eq!(arr, [5, 1, 2, 3, 4]);

        rotate_array(&mut arr, 2);
        assert_eq!(arr, [3, 4, 5, 1, 2]);

        rotate_array(&mut arr, 2);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        let mut arr = (1..100).collect::<Vec<_>>();

        rotate_array(&mut arr, 20);
        let mut expected = (80..100).collect::<Vec<_>>();
        expected.append(&mut (1..80).collect());
        assert_eq!(arr, expected);

        rotate_array(&mut arr, 79);
        assert_eq!(arr, (1..100).collect::<Vec<_>>());
    }

    #[test]
    fn test_edge_cases() {
        let mut arr = vec![];
        rotate_array(&mut arr, 1);
        assert_eq!(arr, []);

        let mut arr = vec![1];
        rotate_array(&mut arr, 1);
        assert_eq!(arr, [1]);

        let mut arr = vec![1];
        rotate_array(&mut arr, 100);
        assert_eq!(arr, [1]);

        let mut arr = (1..10).collect::<Vec<_>>();
        rotate_array(&mut arr, 99);
        assert_eq!(arr, (1..10).collect::<Vec<_>>());

        let mut arr = vec![0, 1];
        rotate_array(&mut arr, usize::MAX);
        assert_eq!(arr, [1, 0]);
    }
}

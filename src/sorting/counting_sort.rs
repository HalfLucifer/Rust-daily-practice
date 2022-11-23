/*
  Counting sort
  - time complexity O(n + k)
  - space complexity O(n + k)

  where k is the range between min element and max element
*/
pub fn counting_sort(arr: &mut [i32]) {
    let max = arr.iter().max().unwrap_or(&0).clone();
    let min = arr.iter().min().unwrap_or(&0).clone();
    let mut prefix_sum = vec![0; (max - min) as usize + 1];

    // Find the count for each element
    arr.iter().for_each(|v| {
        prefix_sum[(*v - min) as usize] += 1;
    });

    // Update prefix sum
    for i in 1..prefix_sum.len() {
        prefix_sum[i] += prefix_sum[i - 1];
    }

    for value in arr.to_vec().into_iter() {
        // Get index by prefix sum
        let index = prefix_sum[(value - min) as usize] - 1;
        // Ouput value to input array
        arr[index] = value;
        // Decrement prefix sum
        prefix_sum[(value - min) as usize] -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut arr = (0..=10).collect::<Vec<_>>();
        counting_sort(&mut arr);
        assert_eq!(arr, (0..=10).collect::<Vec<_>>());

        let mut arr = (0..=10).rev().collect::<Vec<_>>();
        counting_sort(&mut arr);
        assert_eq!(arr, (0..=10).collect::<Vec<_>>());

        let mut arr = [5, 5, 5, 5, 5, 4, 4, 4, 4, 1, 2, 2, 3, 3, 3];
        let mut expected = arr.clone();
        counting_sort(&mut arr);
        expected.sort();
        assert_eq!(arr, expected);

        let mut arr = [-8, 1, 10, 14, -3, -11, 7, 4, 5, -2, -6];
        let mut expected = arr.clone();
        counting_sort(&mut arr);
        expected.sort();
        assert_eq!(arr, expected);

        let mut arr = (-100..100).rev().collect::<Vec<_>>();
        let mut expected = arr.clone();
        counting_sort(&mut arr);
        expected.sort();
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_edge_cases() {
        let mut arr = [];
        counting_sort(&mut arr);
        assert_eq!(arr, []);

        let mut arr = [0];
        counting_sort(&mut arr);
        assert_eq!(arr, [0]);

        let mut arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        counting_sort(&mut arr);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}

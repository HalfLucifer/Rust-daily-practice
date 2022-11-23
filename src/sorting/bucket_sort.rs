pub fn bucket_sort(arr: &mut [u32], bucket_count: usize) {
    assert!(bucket_count > 0);

    if arr.len() <= 1 {
        return;
    }

    // Find max & min to determine value range
    let max = arr.iter().max().unwrap_or(&0).clone();
    let min = arr.iter().min().unwrap_or(&0).clone();
    let range = (max - min + 1) / bucket_count as u32;

    if range == 0 {
        return;
    }

    let mut bucket = vec![vec![]; bucket_count];
    let mut output_index = 0;

    // Put each value into buckets
    arr.iter().for_each(|v| {
        let mut bucket_index = ((*v - min) / range) as usize;

        if bucket_index >= bucket_count {
            bucket_index = bucket_count - 1;
        }

        bucket[bucket_index].push(*v);
    });

    bucket.iter_mut().for_each(|b| {
        // Sort each bucket
        b.sort();
        // Output values
        b.iter().for_each(|v| {
            arr[output_index] = *v;
            output_index += 1;
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let expected = (0..100).collect::<Vec<_>>();

        let mut arr = (0..100).rev().collect::<Vec<_>>();
        bucket_sort(&mut arr, 2);
        assert_eq!(arr, expected);

        let mut arr = (0..100).rev().collect::<Vec<_>>();
        bucket_sort(&mut arr, 3);
        assert_eq!(arr, expected);

        let mut arr = (0..100).rev().collect::<Vec<_>>();
        bucket_sort(&mut arr, 10);
        assert_eq!(arr, expected);

        let mut arr = (0..100).rev().collect::<Vec<_>>();
        bucket_sort(&mut arr, 50);
        assert_eq!(arr, expected);

        let mut arr = (0..100).rev().collect::<Vec<_>>();
        bucket_sort(&mut arr, 100);
        assert_eq!(arr, expected);

        let mut arr = [666666, 55555, 4444, 333, 22, 1, 7777777];
        bucket_sort(&mut arr, 3);
        assert_eq!(arr, [1, 22, 333, 4444, 55555, 666666, 7777777]);

        let mut arr = [55, 3, 91, 47, 8, 15, 12, 8, 66];
        bucket_sort(&mut arr, 4);
        assert_eq!(arr, [3, 8, 8, 12, 15, 47, 55, 66, 91]);

        let mut arr = [2310, 1230, 694, 222, 33, 4, 5566, 918, 77];
        bucket_sort(&mut arr, 5);
        assert_eq!(arr, [4, 33, 77, 222, 694, 918, 1230, 2310, 5566]);
    }

    #[test]
    fn test_edge_cases() {
        let mut arr = [];
        bucket_sort(&mut arr, 1);
        assert_eq!(arr, []);

        let mut arr = [0];
        bucket_sort(&mut arr, 1);
        assert_eq!(arr, [0]);

        let mut arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        bucket_sort(&mut arr, 1);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        bucket_sort(&mut arr, 10);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}

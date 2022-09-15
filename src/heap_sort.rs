pub fn heap_sort(arr: &mut [i32]) {
    // Construct a max binary heap
    for i in (0..arr.len() / 2).rev() {
        heapify(arr, i, arr.len());
    }

    for i in (1..arr.len()).rev() {
        if arr[0] > arr[i] {
            // Swap the largest value to end of array
            arr.swap(0, i);

            // Heapify the remaining unsorted pile
            heapify(arr, 0, i - 1);
        }
    }
}

fn heapify(arr: &mut [i32], index: usize, size: usize) {
    let mut larger_index = index;
    let left_child = index * 2 + 1;
    let right_child = index * 2 + 2;

    if left_child < size && arr[left_child] > arr[larger_index] {
        larger_index = left_child;
    }

    if right_child < size && arr[right_child] > arr[larger_index] {
        larger_index = right_child;
    }

    if index != larger_index {
        arr.swap(index, larger_index);
        heapify(arr, larger_index, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut arr = (1..=10).collect::<Vec<_>>();
        heap_sort(&mut arr);
        assert_eq!(arr, (1..=10).collect::<Vec<_>>());

        let mut arr = (1..=100).rev().collect::<Vec<_>>();
        let mut expected = arr.clone();
        expected.sort();
        heap_sort(&mut arr);
        assert_eq!(arr, expected);

        let mut arr = vec![100, -2, 33, -41, 0, -99, 12, -23, 24, -55];
        let mut expected = arr.clone();
        expected.sort();
        heap_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_edge_cases() {
        let arr = &mut vec![];
        heap_sort(arr);
        assert_eq!(arr, &[]);

        let arr = &mut vec![0];
        heap_sort(arr);
        assert_eq!(arr, &[0]);

        let arr = &mut vec![i32::MAX, i32::MIN, 0];
        heap_sort(arr);
        assert_eq!(arr, &[i32::MIN, 0, i32::MAX]);

        let arr = &mut vec![1, 0, 0, 0, 0, 0, 0, 0, 0, -1];
        heap_sort(arr);
        assert_eq!(arr, &[-1, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    }
}

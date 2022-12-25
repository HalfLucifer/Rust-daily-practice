pub fn build_max_binary_heap(arr: &mut [i32]) {
    fn max_heapify(arr: &mut [i32], index: usize) {
        let left_child = index * 2 + 1;
        let right_child = index * 2 + 2;
        let mut larger_index = index;

        // Find the index of larger children
        if left_child < arr.len() && arr[left_child] > arr[larger_index] {
            larger_index = left_child;
        }

        if right_child < arr.len() && arr[right_child] > arr[larger_index] {
            larger_index = right_child;
        }

        if index != larger_index {
            arr.swap(index, larger_index);
            max_heapify(arr, larger_index);
        }
    }

    // Heapify only non-leaf nodes (from index 0 to len/2 - 1)
    for i in (0..arr.len() / 2).rev() {
        max_heapify(arr, i);
    }
}

pub fn insert_to_max_heap(arr: &mut Vec<i32>, value: i32) {
    arr.push(value);

    let mut index = arr.len() as i32 - 1;
    let mut parent = (index - 1) / 2;

    while parent >= 0 && arr[index as usize] > arr[parent as usize] {
        arr.swap(index as usize, parent as usize);
        index = parent;
        parent = (index - 1) / 2;
    }
}

pub fn validate_max_binary_heap(arr: &[i32]) -> bool {
    for i in (0..arr.len() / 2).rev() {
        let left = i * 2 + 1;
        let right = i * 2 + 2;

        if (left < arr.len() && arr[i] < arr[left]) || (right < arr.len() && arr[i] < arr[right]) {
            return false;
        }
    }

    true
}

pub fn build_min_binary_heap(arr: &mut [i32]) {
    fn min_heapify(arr: &mut [i32], index: usize) {
        let left_child = index * 2 + 1;
        let right_child = index * 2 + 2;
        let mut smaller_index = index;

        // Find the index of smaller children
        if left_child < arr.len() && arr[left_child] < arr[smaller_index] {
            smaller_index = left_child;
        }

        if right_child < arr.len() && arr[right_child] < arr[smaller_index] {
            smaller_index = right_child;
        }

        if index != smaller_index {
            arr.swap(index, smaller_index);
            min_heapify(arr, smaller_index);
        }
    }

    for i in (0..arr.len() / 2).rev() {
        min_heapify(arr, i);
    }
}

pub fn insert_to_min_heap(arr: &mut Vec<i32>, value: i32) {
    arr.push(value);

    let mut index = arr.len() as i32 - 1;
    let mut parent = (index - 1) / 2;

    while parent >= 0 && arr[index as usize] < arr[parent as usize] {
        arr.swap(index as usize, parent as usize);
        index = parent;
        parent = (index - 1) / 2;
    }
}

pub fn validate_min_binary_heap(arr: &[i32]) -> bool {
    for i in (0..arr.len() / 2).rev() {
        let left = i * 2 + 1;
        let right = i * 2 + 2;

        if (left < arr.len() && arr[i] > arr[left]) || (right < arr.len() && arr[i] > arr[right]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_heap_cases() {
        let arr = &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        build_max_binary_heap(arr);

        assert!(validate_max_binary_heap(arr));
        assert_eq!(arr, &[10, 9, 7, 8, 5, 6, 3, 1, 4, 2]);
    }

    #[test]
    fn test_min_heap_cases() {
        let arr = &mut vec![29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1];
        build_min_binary_heap(arr);

        assert!(validate_min_binary_heap(arr));
        assert_eq!(
            arr,
            &[1, 9, 3, 13, 11, 5, 17, 15, 23, 27, 21, 7, 19, 25, 29]
        );
    }

    #[test]
    fn test_insert_value_cases() {
        let arr = &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        build_max_binary_heap(arr);
        insert_to_max_heap(arr, 100);

        assert!(validate_max_binary_heap(arr));
        assert_eq!(arr, &[100, 10, 7, 8, 9, 6, 3, 1, 4, 2, 5]);

        build_min_binary_heap(arr);
        insert_to_min_heap(arr, 50);

        assert!(validate_min_binary_heap(arr));
        assert_eq!(arr, &[1, 2, 3, 4, 5, 6, 7, 8, 10, 9, 100, 50]);
    }

    #[test]
    fn test_insert_from_empty_cases() {
        let mut arr = vec![];
        for i in 0..100 {
            insert_to_max_heap(&mut arr, i);
        }
        assert!(validate_max_binary_heap(&arr));

        let mut arr = vec![];
        for i in (0..100).rev() {
            insert_to_min_heap(&mut arr, i);
        }
        assert!(validate_min_binary_heap(&arr));
    }

    #[test]
    fn test_validation_cases() {
        let arr = [0, 0, 0, 0, 0, 0, 0];
        assert!(validate_max_binary_heap(&arr));
        assert!(validate_min_binary_heap(&arr));

        let arr = [100, 90, 95, 90, 89, 87, 95];
        assert!(validate_max_binary_heap(&arr));

        let arr = [100, 90, 95, 90, 89, 87, 96];
        assert!(!validate_max_binary_heap(&arr));

        let arr = [-100, -90, -95, -90, -89, -87, -95];
        assert!(validate_min_binary_heap(&arr));

        let arr = [-100, -90, -95, -90, -89, -87, -96];
        assert!(!validate_min_binary_heap(&arr));
    }

    #[test]
    fn test_edge_cases() {
        let arr = &mut vec![];
        build_max_binary_heap(arr);
        assert_eq!(arr, &[]);

        let arr = &mut vec![0];
        build_max_binary_heap(arr);
        assert_eq!(arr, &[0]);

        let arr = &mut vec![i32::MIN, i32::MAX];
        build_max_binary_heap(arr);
        assert_eq!(arr, &[i32::MAX, i32::MIN]);
    }
}

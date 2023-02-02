pub fn find_median_in_two_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mid = (len1 + len2) / 2;

    if (len1 + len2) % 2 == 0 {
        // Even element count, find 2 middle elements to divide
        (find_kth_element(&nums1, &nums2, mid) + find_kth_element(&nums1, &nums2, mid + 1)) as f64 * 0.5
    } else {
        // Odd element count
        find_kth_element(&nums1, &nums2, mid + 1) as f64
    }
}

fn find_kth_element(nums1: &[i32], nums2: &[i32], target: usize) -> i32 {
    let len1 = nums1.len();
    let len2 = nums2.len();

    // Swap num1 and num2 to make sure count of num1 is lesser
    if len1 > len2 {
        return find_kth_element(nums2, nums1, target);
    }

    // nums1 is depleted, found the target in nums2
    if len1 == 0 {
        return nums2[target as usize - 1];
    }

    // target is recursively shrink to 1, found it either in nums1 or in nums2
    if target == 1 {
        return nums1[0].min(nums2[0]);
    }

    // Determine the next index
    let next1 = len1.min(target / 2);
    let next2 = target - next1;

    let mid1 = nums1[next1 - 1];
    let mid2 = nums2[next2 - 1];

    if mid1 < mid2 {
        // Slice nums1 off
        find_kth_element(&nums1[next1..], nums2, target - next1)
    } else {
        // Slice nums2 off
        find_kth_element(nums1, &nums2[next2..], target - next2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(
            find_median_in_two_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );

        assert_eq!(
            find_median_in_two_sorted_arrays(vec![1, 2, 3, 4, 6, 7, 8, 9], vec![5]),
            5.0
        );

        assert_eq!(
            find_median_in_two_sorted_arrays((1..=100).collect::<Vec<_>>(), vec![101]),
            51.0
        );

        assert_eq!(
            find_median_in_two_sorted_arrays(vec![-10000], (1..=100).collect::<Vec<_>>()),
            50.0
        );

        assert_eq!(
            find_median_in_two_sorted_arrays(
                (100..=200).collect::<Vec<_>>(),
                (-200..=-100).collect::<Vec<_>>()
            ),
            0.0
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_median_in_two_sorted_arrays(vec![0], vec![0]), 0.0);
        assert_eq!(find_median_in_two_sorted_arrays(vec![0], vec![1]), 0.5);
        assert_eq!(find_median_in_two_sorted_arrays(vec![-1], vec![1]), 0.0);
        assert_eq!(
            find_median_in_two_sorted_arrays(vec![i32::MIN], vec![i32::MAX]),
            -0.5
        );
    }
}

/*
   Given two integer arrays nums1 and nums2, return an array of their intersection.
   Each element in the result must be unique
*/
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut arr = vec![false; 1001];
    let mut res = vec![];

    for n in nums1 {
        arr[n as usize] = true;
    }

    for n in nums2 {
        if arr[n as usize] {
            arr[n as usize] = false;
            res.push(n);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), [2]);
        assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), [9, 4]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(intersection(vec![0], vec![1]), []);
        assert_eq!(intersection(vec![0], vec![0]), [0]);
    }
}

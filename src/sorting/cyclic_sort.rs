/*
    Given an array of size n, all elements are distinct and within [0..n],
    sorting them is ascending order

    Cyclic sort
    - Time complexity: O(n)
    - Space complexity: O(1)
*/
pub fn cyclic_sort(nums: &mut Vec<i32>) {
    let mut i = 0;
    let n = nums.len();

    while i < n {
        let curr = nums[i];

        if curr < n as i32 && curr != nums[curr as usize] {
            nums.swap(curr as usize, i);
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut nums = vec![2, 0, 1];
        cyclic_sort(&mut nums);
        assert_eq!(nums, [0, 1, 2]);

        let mut nums = vec![4, 6, 2, 8, 3, 5, 9, 0, 1, 7];
        cyclic_sort(&mut nums);
        assert_eq!(nums, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut nums = (0..100).collect::<Vec<i32>>();
        cyclic_sort(&mut nums);
        assert_eq!(nums, (0..100).collect::<Vec<i32>>());
    }

    #[test]
    fn test_edge_cases() {
        let mut nums = vec![];
        cyclic_sort(&mut nums);
        assert_eq!(nums, []);

        let mut nums = vec![0];
        cyclic_sort(&mut nums);
        assert_eq!(nums, [0]);
    }
}

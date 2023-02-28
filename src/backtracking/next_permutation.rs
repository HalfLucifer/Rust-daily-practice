/*
    The next permutation of an array of integers is the next lexicographically greater
    permutation of its integer.

    If such arrangement is not possible, the array must be rearranged as the lowest
    possible order.
*/
pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut i = nums.len() - 1;

    // Find the first descending element started from end
    while i > 0 && nums[i] <= nums[i - 1] {
        i -= 1;
    }

    // List is strictly descending, sort it to get the lowest possible order
    if i == 0 {
        nums.sort_unstable();
        return;
    }

    // Decrement 1 to the index of first descending element
    i -= 1;

    for j in (i..nums.len()).rev() {
        // Find the first element larger than nums[i]
        if nums[i] < nums[j] {
            // Swap i-th and j-th elements
            nums.swap(i, j);

            // Sort remaining elements
            nums[i + 1..].sort_unstable();

            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut v = vec![1, 2, 3];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 3, 2]);

        let mut v = vec![3, 2, 1];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 2, 3]);

        let mut v = vec![1, 1, 5];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 5, 1]);

        let mut v = vec![9, 7, 8, 1, 5, 4, 3, 2];
        next_permutation(&mut v);
        assert_eq!(v, vec![9, 7, 8, 2, 1, 3, 4, 5]);
    }

    #[test]
    fn test_edge_cases() {
        let mut v = vec![0];
        next_permutation(&mut v);
        assert_eq!(v, vec![0]);

        let mut v = vec![0, 0];
        next_permutation(&mut v);
        assert_eq!(v, vec![0, 0]);

        let mut v = vec![0, 1];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 0]);
    }
}

/*
    Given an array of integers nums containing n + 1 integers where each integer
    is in the range [1, n] inclusive. There is only one repeated number in nums,
    return this repeated number.
*/

// Bit manipulation
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut bit_max = 31;
    let n = nums.len();

    // Calculate max binary length within [1, n]
    while (n - 1) >> bit_max == 0 {
        bit_max -= 1;
    }

    for bit in 0..=bit_max {
        let mask = 1 << bit;
        let mut count = 0;

        for i in 0..n {
            // If nums[i] has a 1 in the bit position
            if nums[i] & mask > 0 {
                count += 1;
            }

            // If i has a 1 in the bit position
            if i as i32 & mask > 0 {
                count -= 1;
            }
        }

        // Found an extra 1 in the bit position
        if count > 0 {
            res |= mask;
        }
    }

    res
}

// Binary search
pub fn find_duplicate_bs(nums: Vec<i32>) -> i32 {
    let mut lo = 1;
    let mut hi = nums.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mut count = 0;

        for i in 0..nums.len() {
            if nums[i] <= mid as i32 {
                count += 1;
            }
        }

        if count <= mid {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    lo as i32
}

// Two pointers
pub fn find_duplicate_tp(nums: Vec<i32>) -> i32 {
    let mut slow = 0;
    let mut fast = 0;

    loop {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];

        if slow == fast {
            break;
        }
    }

    slow = 0;

    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    slow
}

// Mark visited numbers
pub fn find_duplicate_by_mark(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();

    for i in 0..n {
        let v = nums[i].abs() as usize;

        if nums[v] < 0 {
            return nums[i].abs();
        }

        nums[v] *= -1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let v = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(v.clone()), 2);
        assert_eq!(find_duplicate_tp(v.clone()), 2);
        assert_eq!(find_duplicate_bit(v.clone()), 2);
        assert_eq!(find_duplicate_by_mark(v.clone()), 2);

        let v = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(v.clone()), 3);
        assert_eq!(find_duplicate_tp(v.clone()), 3);
        assert_eq!(find_duplicate_bit(v.clone()), 3);
        assert_eq!(find_duplicate_by_mark(v.clone()), 3);

        let v = vec![2, 2, 2, 2, 2];
        assert_eq!(find_duplicate(v.clone()), 2);
        assert_eq!(find_duplicate_tp(v.clone()), 2);
        assert_eq!(find_duplicate_bit(v.clone()), 2);
        assert_eq!(find_duplicate_by_mark(v.clone()), 2);

        let v = vec![1, 4, 4, 2, 4];
        assert_eq!(find_duplicate(v.clone()), 4);
        assert_eq!(find_duplicate_tp(v.clone()), 4);
        assert_eq!(find_duplicate_bit(v.clone()), 4);
        assert_eq!(find_duplicate_by_mark(v.clone()), 4);
    }

    #[test]
    fn test_edge_cases() {
        let v = vec![1, 1];
        assert_eq!(find_duplicate(v.clone()), 1);
        assert_eq!(find_duplicate_tp(v.clone()), 1);
        assert_eq!(find_duplicate_bit(v.clone()), 1);
        assert_eq!(find_duplicate_by_mark(v.clone()), 1);

        let v = vec![1, 1, 1];
        assert_eq!(find_duplicate(v.clone()), 1);
        assert_eq!(find_duplicate_tp(v.clone()), 1);
        assert_eq!(find_duplicate_bit(v.clone()), 1);
        assert_eq!(find_duplicate_by_mark(v.clone()), 1);
    }
}

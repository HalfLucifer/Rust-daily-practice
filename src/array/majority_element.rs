use std::collections::HashMap;

/*
    The majority element is the element that appears more than ⌊n / 2⌋ times

    Boyer-Moore algorithm
    - time complexity: O(n)
    - space complexity: O(1)

    NOTE: this method can't handle the case of majority element inexistence
*/
pub fn majority_element(nums: &[i32]) -> i32 {
    let mut majority = 0;
    let mut count = 0;

    for n in nums {
        if count == 0 {
            majority = *n;
        }

        if majority == *n {
            count += 1;
        } else {
            count -= 1;
        }
    }

    majority
}

/*
    Hashmap method
    - time complexity: O(n)
    - space complexity: O(n)
*/
pub fn majority_element_hashmap(nums: &[i32]) -> Option<i32> {
    let mut hm = HashMap::new();
    let threshold = nums.len() / 2;

    nums.iter().for_each(|n| {
        hm.entry(n).and_modify(|e| *e += 1).or_insert(1);
    });

    for (k, v) in hm {
        if v > threshold {
            return Some(*k);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let arr = [1, 1, 1, 0, -1];
        assert_eq!(majority_element(&arr), 1);
        assert_eq!(majority_element_hashmap(&arr), Some(1));

        let arr = [1, 0, -1, -1, -1];
        assert_eq!(majority_element(&arr), -1);
        assert_eq!(majority_element_hashmap(&arr), Some(-1));

        let arr = [1, 0, 0, 0, -1];
        assert_eq!(majority_element(&arr), 0);
        assert_eq!(majority_element_hashmap(&arr), Some(0));
    }

    #[test]
    fn test_edge_cases() {
        let arr = [0];
        assert_eq!(majority_element(&arr), 0);
        assert_eq!(majority_element_hashmap(&arr), Some(0));

        let arr = [0, 0];
        assert_eq!(majority_element(&arr), 0);
        assert_eq!(majority_element_hashmap(&arr), Some(0));

        let arr = [0, 1];
        assert_eq!(majority_element_hashmap(&arr), None);

        let arr = [0, 0, 0, 1, 1, 1];
        assert_eq!(majority_element_hashmap(&arr), None);

        let arr = [0, 1, 2, 0, 1, 2];
        assert_eq!(majority_element_hashmap(&arr), None);
    }
}

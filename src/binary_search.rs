use std::cmp::Ordering;

pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Result<usize, usize> {
    if arr.is_empty() {
        return Err(0);
    }

    let mut start: isize = 0;
    let mut end: isize = arr.len() as isize - 1;

    while end >= start {
        // Preventing from integer overflow
        let mid = start + (end - start) / 2;

        match target.cmp(&arr[mid as usize]) {
            Ordering::Greater => {
                start = mid + 1;
            }

            Ordering::Less => {
                end = mid - 1;
            }

            Ordering::Equal => {
                return Ok(mid as usize);
            }
        }
    }

    Err((end + 1) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let a = (1..100).collect::<Vec<_>>();

        for i in 1..100 {
            assert_eq!(binary_search(&a, &i), Ok(i - 1));
        }
    }

    #[test]
    fn test_edge_cases() {
        let a = [];
        assert_eq!(binary_search(&a, &0), Err(0));

        let a = [0];
        assert_eq!(binary_search(&a, &0), Ok(0));

        let a = [i32::MIN, 0, i32::MAX];
        assert_eq!(binary_search(&a, &i32::MIN), Ok(0));
        assert_eq!(binary_search(&a, &0), Ok(1));
        assert_eq!(binary_search(&a, &i32::MAX), Ok(2));
    }

    #[test]
    fn test_absent_cases() {
        let mut a = (1..=100).collect::<Vec<_>>();

        a.remove(49);
        assert_eq!(binary_search(&a, &0), Err(0));
        assert_eq!(binary_search(&a, &50), Err(49));
        assert_eq!(binary_search(&a, &101), Err(99));

        a.remove(0);
        assert_eq!(binary_search(&a, &0), Err(0));
        assert_eq!(binary_search(&a, &1), Err(0));
        assert_eq!(binary_search(&a, &101), Err(98));
    }
}

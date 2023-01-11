/*
   Sort pancakes from smallest to largest
   - input: a pile of pancakes by size
   - output: each time flipping pancake index

   NOTE: the solution is not the minimum count of flip
*/
pub fn sort_pancake(arr: &mut Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    sort_pancake_recursive(arr, arr.len(), &mut result);
    result
}

fn sort_pancake_recursive(arr: &mut Vec<usize>, count: usize, result: &mut Vec<usize>) {
    if count <= 1 {
        return;
    }

    // Find the largest pancake
    let max = arr
        .iter()
        .enumerate()
        .filter(|(i, _)| *i < count)
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap_or((0, &0));

    let max_index = max.0;
    if max_index != count - 1 {
        if max_index > 0 {
            // Flip pancakes from max index
            arr[0..=max_index].reverse();
            result.push(max_index);
        }

        // Flip pancakes from count-1
        arr[0..=count - 1].reverse();
        result.push(count - 1);
    }

    sort_pancake_recursive(arr, count - 1, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(sort_pancake(&mut vec![1, 2, 3, 4, 5]), []);
        assert_eq!(sort_pancake(&mut vec![5, 4, 3, 2, 1]), [4]);
        
        assert_eq!(sort_pancake(&mut vec![2, 1, 4, 3]), [2, 3, 2]);
        assert_eq!(sort_pancake(&mut vec![3, 2, 4, 1]), [2, 3, 1, 2, 1]);
        assert_eq!(sort_pancake(&mut vec![1, 4, 3, 2]), [1, 3, 1, 2]);

        let mut arr = vec![2, 5, 3, 7, 1, 6, 4, 8, 9];
        sort_pancake(&mut arr);
        assert_eq!(arr, (1..10).collect::<Vec<_>>());
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(sort_pancake(&mut vec![]), []);
        assert_eq!(sort_pancake(&mut vec![0]), []);
        assert_eq!(sort_pancake(&mut vec![1]), []);
        assert_eq!(sort_pancake(&mut vec![2, 1]), [1]);
        assert_eq!(sort_pancake(&mut vec![1, 2, 2]), []);
        assert_eq!(sort_pancake(&mut vec![2, 2, 1]), [1, 2]);
    }
}

pub fn largest_histogram_rectangle(arr: &[u32]) -> u32 {
    let mut result = 0;
    let mut queue = vec![];
    let mut heights = arr.to_vec();

    // Insert 0 to begin and end of array for edge cases
    heights.insert(0, 0);
    heights.push(0);

    for i in 0..heights.len() {
        while let Some(&last) = queue.last() {
            if heights[last] > heights[i] {
                let height = heights[last];

                queue.pop();

                let next = i;
                let prev = queue.last().unwrap_or(&0);
                let width = (next - prev - 1) as u32;

                result = result.max(height * width);
            } else {
                break;
            }
        }

        queue.push(i);
    }

    result as u32
}

pub fn largest_histogram_rectangle_3pass(arr: &[u32]) -> u32 {
    let mut input = arr.to_vec();

    // Insert 0 to begin and end of array for edge cases
    input.insert(0, 0);
    input.push(0);

    // Pass 1: find the previous smaller index
    let pre = pre_smaller_index(&input);

    // Pass 2: find the next smaller index
    let next = next_smaller_index(&input);

    // Pass 3: find the area for each height value
    let mut result = 0;

    for i in 0..input.len() {
        let height = input[i] as i32;
        let width = next[i].unwrap_or(0) as i32 - pre[i].unwrap_or(0) as i32 - 1;
        let area = height * width;
        result = result.max(area as u32);
    }

    result
}

fn pre_smaller_index(arr: &[u32]) -> Vec<Option<usize>> {
    let mut result = vec![None; arr.len()];
    let mut queue = vec![];

    for i in 0..arr.len() {
        let height = arr[i];

        while let Some(&last) = queue.last() {
            if height <= arr[last] {
                queue.pop();
            } else {
                result[i] = Some(last);
                break;
            }
        }

        queue.push(i);
    }

    result
}

fn next_smaller_index(arr: &[u32]) -> Vec<Option<usize>> {
    let mut result = vec![None; arr.len()];
    let mut queue = vec![];

    for i in (0..arr.len()).rev() {
        let height = arr[i];

        while let Some(&last) = queue.last() {
            if height <= arr[last] {
                queue.pop();
            } else {
                result[i] = Some(last);
                break;
            }
        }

        queue.push(i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        /*  Histogram

             |
             |  |
             | ||
             ||||
            |||||

            max rectangle area = 8
        */
        let arr = [1, 5, 2, 3, 4];
        assert_eq!(largest_histogram_rectangle(&arr), 8);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle_3pass(&arr)
        );

        let arr = [2, 1, 5, 6, 2, 3];
        assert_eq!(largest_histogram_rectangle(&arr), 10);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle_3pass(&arr)
        );

        let arr = [2, 4];
        assert_eq!(largest_histogram_rectangle(&arr), 4);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle_3pass(&arr)
        );

        let arr = [2, 3, 3, 1, 2, 1];
        assert_eq!(largest_histogram_rectangle(&arr), 6);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle_3pass(&arr)
        );

        let arr = [1, 2, 1, 2, 1];
        assert_eq!(largest_histogram_rectangle(&arr), 5);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle_3pass(&arr)
        );

        let arr = [1, 2, 3, 2, 1];
        assert_eq!(largest_histogram_rectangle(&arr), 6);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle_3pass(&arr)
        );

        let arr = [1, 1, 1, 1, 101, 1, 1, 1, 1];
        assert_eq!(largest_histogram_rectangle(&arr), 101);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle_3pass(&arr)
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(largest_histogram_rectangle(&[]), 0);
        assert_eq!(largest_histogram_rectangle(&[0]), 0);
        assert_eq!(largest_histogram_rectangle(&[1]), 1);

        assert_eq!(largest_histogram_rectangle(&[0; 100]), 0);
        assert_eq!(largest_histogram_rectangle(&[1; 100]), 100);
    }

    #[test]
    fn test_ascending_and_descending_cases() {
        assert_eq!(largest_histogram_rectangle(&[1, 2, 3, 4, 5]), 9);
        assert_eq!(
            largest_histogram_rectangle(&[1, 2, 3, 4, 5]),
            largest_histogram_rectangle(&[5, 4, 3, 2, 1])
        );

        let arr = (1..=50).collect::<Vec<_>>();
        let arr_rev = (1..=50).rev().collect::<Vec<_>>();
        assert_eq!(largest_histogram_rectangle(&arr), 650);
        assert_eq!(
            largest_histogram_rectangle(&arr),
            largest_histogram_rectangle(&arr_rev)
        );
    }
}

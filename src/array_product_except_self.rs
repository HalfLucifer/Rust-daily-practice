pub fn array_product_except_self(nums: &Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len == 1 {
        return nums.clone();
    }

    let mut result = vec![1; len];
    let mut prefix = 1;
    let mut postfix = 1;

    for i in 0..len {
        result[i] *= prefix;
        prefix *= nums[i];

        result[len - 1 - i] *= postfix;
        postfix *= nums[len - 1 - i];
    }

    result
}

pub fn array_product_except_self_3pass(nums: &Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len == 1 {
        return nums.clone();
    }

    let mut result = vec![0; len];
    let mut prefix = vec![0; len];
    let mut postfix = vec![0; len];

    prefix[0] = nums[0];
    postfix[len - 1] = nums[len - 1];

    for i in 1..len {
        prefix[i] = prefix[i - 1] * nums[i];
    }

    for i in (0..len - 1).rev() {
        postfix[i] = postfix[i + 1] * nums[i];
    }

    for i in 0..len {
        let pre = { if i == 0 { 1 } else { prefix[i - 1] } };
        let post = { if i == len - 1 { 1 } else { postfix[i + 1] } };
        result[i] = pre * post;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(array_product_except_self(&vec![10, 20, 30]), [600, 300, 200]);
        assert_eq!(array_product_except_self(&vec![1, 2, 3, 4, 5]), [120, 60, 40, 30, 24]);
        assert_eq!(array_product_except_self(&vec![-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);

        assert_eq!(array_product_except_self_3pass(&vec![10, 20, 30]), [600, 300, 200]);
        assert_eq!(array_product_except_self_3pass(&vec![1, 2, 3, 4, 5]), [120, 60, 40, 30, 24]);
        assert_eq!(array_product_except_self_3pass(&vec![-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);

        let arr = (1..10).collect::<Vec<_>>();
        assert_eq!(array_product_except_self(&arr), array_product_except_self_3pass(&arr));

        let arr = vec![10, -3, 5, 7, -2, 81, 4, -9];
        assert_eq!(array_product_except_self(&arr), array_product_except_self_3pass(&arr));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(array_product_except_self(&vec![0]), [0]);
        assert_eq!(array_product_except_self(&vec![0, 0]), [0, 0]);
        assert_eq!(array_product_except_self(&vec![0, 1]), [1, 0]);
        assert_eq!(array_product_except_self(&vec![1, 0, 1]), [0, 1, 0]);

        assert_eq!(array_product_except_self_3pass(&vec![0]), [0]);
        assert_eq!(array_product_except_self_3pass(&vec![0, 0]), [0, 0]);
        assert_eq!(array_product_except_self_3pass(&vec![0, 1]), [1, 0]);
        assert_eq!(array_product_except_self_3pass(&vec![1, 0, 1]), [0, 1, 0]);
    }
}

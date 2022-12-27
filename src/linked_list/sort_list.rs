/*
    Method 1. using merge sort
*/
pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let len = length(&head);
    sort_list_recursively(&mut head, len)
}

/*
   Method 2. using array to sort
*/
pub fn sort_list_with_array(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut v = vec![];

    while let Some(node) = head {
        v.push(node.val);
        head = node.next;
    }

    // Sort by descending order
    v.sort_by(|a, b| b.cmp(a));
    
    v.into_iter().fold(None, |next, val| Some(Box::new(ListNode { val, next })))
}

fn sort_list_recursively(head: &mut Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
    if len <= 1 {
        return head.take();
    }

    let half = len / 2;
    let mut mid = nth_node_iteratively(head, half);

    let left = sort_list_recursively(head, half);
    let right = sort_list_recursively(&mut mid, len - half);

    merge_sorted_lists(left, right)
}

fn length(head: &Option<Box<ListNode>>) -> usize {
    let mut len = 0;
    let mut curr = head;

    while let Some(node) = curr {
        len += 1;
        curr = &node.next;
    }

    len
}

fn nth_node_iteratively(head: &mut Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
    let mut curr = head;

    for _ in 0..n {
        curr = &mut curr.as_mut().unwrap().next;

        if curr.is_none() {
            return None;
        }
    }

    curr.take()
}

fn merge_sorted_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,

        (Some(n), None) | (None, Some(n)) => Some(n),

        (Some(l1), Some(l2)) => {
            if l1.val > l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_sorted_lists(Some(l1), l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_sorted_lists(l1.next, Some(l2)),
                }))
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn traverse(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut list = list;

    while let Some(node) = list {
        result.push(node.val);
        list = &node.next;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut a1 = ListNode::new(75);
        let mut a2 = ListNode::new(20);
        let mut a3 = ListNode::new(81);
        let mut a4 = ListNode::new(49);
        let a5 = ListNode::new(56);

        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let sorted1 = sort_list(Some(Box::new(a1.clone())));
        let sorted2 = sort_list_with_array(Some(Box::new(a1.clone())));

        assert_eq!(traverse(&sorted1), [20, 49, 56, 75, 81]);
        assert_eq!(sorted1, sorted2);
    }

    #[test]
    fn test_descending_cases() {
        let mut a1 = ListNode::new(99);
        let mut a2 = ListNode::new(88);
        let mut a3 = ListNode::new(77);
        let mut a4 = ListNode::new(66);
        let a5 = ListNode::new(55);

        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let sorted1 = sort_list(Some(Box::new(a1.clone())));
        let sorted2 = sort_list_with_array(Some(Box::new(a1.clone())));

        assert_eq!(traverse(&sorted1), [55, 66, 77, 88, 99]);
        assert_eq!(sorted1, sorted2);
    }

    #[test]
    fn test_ascending_cases() {
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let mut a3 = ListNode::new(3);
        let mut a4 = ListNode::new(4);
        let a5 = ListNode::new(5);

        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let sorted1 = sort_list(Some(Box::new(a1.clone())));
        let sorted2 = sort_list_with_array(Some(Box::new(a1.clone())));

        assert_eq!(traverse(&sorted1), [1, 2, 3, 4, 5]);
        assert_eq!(sorted1, sorted2);
    }

    #[test]
    fn test_edge_cases() {
        let a1 = ListNode::new(0);
        let sorted1 = sort_list(Some(Box::new(a1.clone())));
        let sorted2 = sort_list_with_array(Some(Box::new(a1.clone())));

        assert_eq!(traverse(&sorted1), [0]);
        assert_eq!(sorted1, sorted2);
    }
}

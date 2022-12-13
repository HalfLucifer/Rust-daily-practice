pub fn merge_two_lists(
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
                    next: merge_two_lists(Some(l1), l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                }))
            }
        }
    }
}

pub fn merge_two_lists_iteratively(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }

    if list2.is_none() {
        return list1;
    }

    let mut dummy = ListNode::new(-1);
    let mut curr = &mut dummy;

    let mut l1 = list1;
    let mut l2 = list2;

    // Iterate until any list is empty
    while l1.is_some() && l2.is_some() {
        let v1 = l1.as_ref().unwrap().val;
        let v2 = l2.as_ref().unwrap().val;

        if v1 <= v2 {
            curr.next = l1.clone();
            l1 = l1.unwrap().next;
        } else {
            curr.next = l2.clone();
            l2 = l2.unwrap().next;
        }

        curr = curr.next.as_mut().unwrap();
    }

    // Proceed the remaining non-empty list
    if l1.is_some() {
        curr.next = l1;
    } else if l2.is_some() {
        curr.next = l2;
    }

    dummy.next
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

pub fn traverse(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut list = list;

    while let Some(node) = list {
        result.push(node.val);
        list = node.next;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let a3 = ListNode::new(4);
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));
        let c1 = a1.clone();

        let mut b1 = ListNode::new(1);
        let mut b2 = ListNode::new(3);
        let b3 = ListNode::new(4);
        b2.next = Some(Box::new(b3));
        b1.next = Some(Box::new(b2));
        let d1 = b1.clone();

        let expected = [1, 1, 2, 3, 4, 4];
        let method1 = merge_two_lists(Some(Box::new(a1)), Some(Box::new(b1)));
        let method2 = merge_two_lists_iteratively(Some(Box::new(c1)), Some(Box::new(d1)));

        assert_eq!(traverse(method1), expected);
        assert_eq!(traverse(method2), expected);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(traverse(merge_two_lists(None, None)), vec![]);
        assert_eq!(traverse(merge_two_lists_iteratively(None, None)), vec![]);

        assert_eq!(
            traverse(merge_two_lists(
                Some(Box::new(ListNode::new(0))),
                Some(Box::new(ListNode::new(0)))
            )),
            vec![0, 0]
        );
        assert_eq!(
            traverse(merge_two_lists_iteratively(
                Some(Box::new(ListNode::new(0))),
                Some(Box::new(ListNode::new(0)))
            )),
            vec![0, 0]
        );

        assert_eq!(
            traverse(merge_two_lists(
                Some(Box::new(ListNode::new(0))),
                Some(Box::new(ListNode::new(0)))
            )),
            vec![0, 0]
        );
        assert_eq!(
            traverse(merge_two_lists_iteratively(
                Some(Box::new(ListNode::new(0))),
                Some(Box::new(ListNode::new(0)))
            )),
            vec![0, 0]
        );
    }
}

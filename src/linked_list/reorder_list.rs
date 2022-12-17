/*
   For a list L0 -> L1 -> L2 ... -> Ln-2 -> Ln-1 -> Ln
   Reorder it to  L0 -> Ln -> L1 -> Ln-1 -> L2 -> ...
*/
pub fn reorder_list(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let len = length(&head);
    let mut mid = head.as_mut();

    // Find the middle node
    for _ in 0..len / 2 {
        if let Some(node) = mid {
            mid = node.next.as_mut();
        }
    }

    if let Some(node) = mid {
        // Reverse the list started from middle
        let reversed = reverse(&mut node.next);

        // Merge two list one by one
        if let Some(node) = head {
            node.next = merge_two_lists(reversed, node.next.take(), len - 1);
        }
    }

    head.clone()
}

fn length(head: &Option<Box<ListNode>>) -> i32 {
    let mut len = 0;
    let mut curr = head;

    while let Some(node) = curr {
        curr = &node.next;
        len += 1;
    }

    len
}

fn reverse(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head.take();

    while let Some(mut node) = curr {
        let next = node.next;
        node.next = prev;

        prev = Some(node);
        curr = next;
    }

    prev.take()
}

fn merge_two_lists(
    mut left: Option<Box<ListNode>>,
    right: Option<Box<ListNode>>,
    count: i32,
) -> Option<Box<ListNode>> {
    if count == 0 {
        return None;
    }

    match (left.as_mut(), right.as_ref()) {
        (Some(_), None) => left,
        (None, Some(_)) => right,
        (Some(l), Some(_)) => {
            l.next = merge_two_lists(right, l.next.take(), count - 1);
            left
        }
        _ => None,
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
        let mut a1 = ListNode::new(0);
        let mut a2 = ListNode::new(1);
        let a3 = ListNode::new(2);

        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let re = reorder_list(&mut Some(Box::new(a1)));
        assert_eq!(traverse(&re), [0, 2, 1]);
    }

    #[test]
    fn test_odd_cases() {
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let mut a3 = ListNode::new(3);
        let mut a4 = ListNode::new(4);
        let a5 = ListNode::new(5);

        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let re = reorder_list(&mut Some(Box::new(a1)));
        assert_eq!(traverse(&re), [1, 5, 2, 4, 3]);
    }

    #[test]
    fn test_even_cases() {
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let mut a3 = ListNode::new(3);
        let mut a4 = ListNode::new(4);
        let mut a5 = ListNode::new(5);
        let a6 = ListNode::new(6);

        a5.next = Some(Box::new(a6));
        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let re = reorder_list(&mut Some(Box::new(a1)));
        assert_eq!(traverse(&re), [1, 6, 2, 5, 3, 4]);
    }

    #[test]
    fn test_edge_cases() {
        let mut a1 = ListNode::new(0);
        let re = reorder_list(&mut Some(Box::new(a1.clone())));
        assert_eq!(traverse(&re), [0]);

        let mut a2 = ListNode::new(1);
        a1.next = Some(Box::new(a2));
        let re = reorder_list(&mut Some(Box::new(a1.clone())));
        assert_eq!(traverse(&re), [0, 1]);
    }
}

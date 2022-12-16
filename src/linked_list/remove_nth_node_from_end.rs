pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });

    let mut fast = dummy.clone();
    let mut slow = dummy.as_mut();

    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    while let Some(node) = fast.next {
        fast = node;
        slow = slow.next.as_mut().unwrap();
    }

    slow.next = slow.next.as_ref().unwrap().next.clone();

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
        let mut a3 = ListNode::new(3);
        let mut a4 = ListNode::new(4);
        let a5 = ListNode::new(5);
        
        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        assert_eq!(
            traverse(remove_nth_from_end(Some(Box::new(a1.clone())), 1)),
            [1, 2, 3, 4]
        );
        assert_eq!(
            traverse(remove_nth_from_end(Some(Box::new(a1.clone())), 2)),
            [1, 2, 3, 5]
        );
        assert_eq!(
            traverse(remove_nth_from_end(Some(Box::new(a1.clone())), 3)),
            [1, 2, 4, 5]
        );
        assert_eq!(
            traverse(remove_nth_from_end(Some(Box::new(a1.clone())), 4)),
            [1, 3, 4, 5]
        );
        assert_eq!(
            traverse(remove_nth_from_end(Some(Box::new(a1.clone())), 5)),
            [2, 3, 4, 5]
        );
    }

    #[test]
    fn test_edge_cases() {
        let head = ListNode::new(1);
        assert_eq!(traverse(remove_nth_from_end(Some(Box::new(head)), 1)), []);

        let mut head = ListNode::new(0);
        let tail = ListNode::new(1);
        head.next = Some(Box::new(tail));
        assert_eq!(
            traverse(remove_nth_from_end(Some(Box::new(head.clone())), 1)),
            [0]
        );
        assert_eq!(
            traverse(remove_nth_from_end(Some(Box::new(head.clone())), 2)),
            [1]
        );
    }
}

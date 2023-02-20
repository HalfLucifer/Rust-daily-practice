/*
    Given the head of a linked list and an integer val, remove all the nodes of the linked
    list that has Node.val == val, and return the new head.
*/
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    // Use a dummy to simplify handling of head node
    let mut dummy = Box::new(ListNode::new(-1));
    dummy.next = head;

    let mut curr = dummy.as_mut();

    while let Some(node) = curr.next.take() {
        if node.val == val {
            curr.next = node.next;
        } else {
            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }
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
        let mut a3 = ListNode::new(6);
        let mut a4 = ListNode::new(3);
        let mut a5 = ListNode::new(4);
        let mut a6 = ListNode::new(5);
        let a7 = ListNode::new(6);

        a6.next = Some(Box::new(a7));
        a5.next = Some(Box::new(a6));
        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        assert_eq!(
            traverse(remove_elements(Some(Box::new(a1)), 6)),
            [1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(traverse(remove_elements(None, 1)), []);

        let mut a1 = ListNode::new(7);
        let mut a2 = ListNode::new(7);
        let mut a3 = ListNode::new(7);
        let a4 = ListNode::new(7);
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        assert_eq!(traverse(remove_elements(Some(Box::new(a1)), 7)), []);
    }
}

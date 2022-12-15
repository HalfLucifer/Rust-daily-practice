pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return false;
    }

    // Reverse the list from middle to end
    let mid = get_middle_node(head.clone());
    let reversed = reverse_list(mid);

    let mut front = &head;
    let mut back = &reversed;

    // Compare lists from front and back
    while back.is_some() {
        let front_val = front.as_ref().unwrap().val;
        let back_val = back.as_ref().unwrap().val;

        if front_val != back_val {
            return false;
        }

        front = &front.as_ref().unwrap().next;
        back = &back.as_ref().unwrap().next;
    }

    true
}

fn get_middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.clone()
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut head = head;

    while let Some(mut node) = head {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        head = next;
    }

    prev.take()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let mut a3 = ListNode::new(3);
        let mut a4 = ListNode::new(2);
        let a5 = ListNode::new(1);

        a4.next = Some(Box::new(a5.clone()));
        a3.next = Some(Box::new(a4.clone()));
        a2.next = Some(Box::new(a3.clone()));
        a1.next = Some(Box::new(a2.clone()));

        assert!(is_palindrome(Some(Box::new(a1))));
        assert!(!is_palindrome(Some(Box::new(a2))));
        assert!(!is_palindrome(Some(Box::new(a3))));
        assert!(!is_palindrome(Some(Box::new(a4))));
        assert!(is_palindrome(Some(Box::new(a5))));
    }

    #[test]
    fn test_even_nodes_cases() {
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let mut a3 = ListNode::new(3);
        let mut a4 = ListNode::new(3);
        let mut a5 = ListNode::new(2);
        let a6 = ListNode::new(1);

        a5.next = Some(Box::new(a6.clone()));
        a4.next = Some(Box::new(a5.clone()));
        a3.next = Some(Box::new(a4.clone()));
        a2.next = Some(Box::new(a3.clone()));
        a1.next = Some(Box::new(a2.clone()));

        assert!(is_palindrome(Some(Box::new(a1))));
        assert!(!is_palindrome(Some(Box::new(a2))));
        assert!(!is_palindrome(Some(Box::new(a3))));
        assert!(!is_palindrome(Some(Box::new(a4))));
        assert!(!is_palindrome(Some(Box::new(a5))));
        assert!(is_palindrome(Some(Box::new(a6))));
    }

    #[test]
    fn test_edge_cases() {
        assert!(!is_palindrome(None));

        let a0 = ListNode::new(0);
        assert!(is_palindrome(Some(Box::new(a0))));

        let mut a1 = ListNode::new(0);
        a1.next = Some(Box::new(a1.clone()));
        assert!(is_palindrome(Some(Box::new(a1))));

        let mut a2 = ListNode::new(0);
        let mut a3 = ListNode::new(0);
        let mut a4 = ListNode::new(0);
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        assert!(is_palindrome(Some(Box::new(a2))));
    }
}

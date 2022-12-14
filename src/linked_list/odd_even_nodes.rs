pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr = head;
    let mut odd = ListNode::new(-1);
    let mut even = ListNode::new(-1);

    let mut odd_curr = &mut odd;
    let mut even_curr = &mut even;
    let mut is_even = false;

    while let Some(mut node) = curr {
        // Move cursor to next
        curr = node.next.take();

        if is_even {
            even_curr.next = Some(node);
            even_curr = even_curr.next.as_mut().unwrap();
        } else {
            odd_curr.next = Some(node);
            odd_curr = odd_curr.next.as_mut().unwrap();
        }

        is_even = !is_even;
    }

    // Attach head of even list to tail of odd list
    odd_curr.next = even.next;

    // Return head of odd list
    odd.next
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

        assert_eq!(traverse(odd_even_list(Some(Box::new(a1)))), [1, 3, 5, 2, 4]);
    }

    #[test]
    fn test_edge_cases() {
        let mut a1 = ListNode::new(0);

        let a2 = ListNode::new(1);
        assert_eq!(traverse(odd_even_list(Some(Box::new(a1.clone())))), [0]);

        a1.next = Some(Box::new(a2));
        assert_eq!(traverse(odd_even_list(Some(Box::new(a1.clone())))), [0, 1]);
    }
}

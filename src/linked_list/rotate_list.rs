pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let len = length(&head);
    if len <= 1 {
        return head;
    }

    let k = k % len;
    if k == 0 {
        return head;
    }

    let mut new_head = nth_node(&mut head, len - k);
    append_to_tail(&mut new_head, head);

    new_head
}

fn length(node: &Option<Box<ListNode>>) -> i32 {
    let mut len = 0;
    let mut curr = node;

    while let Some(node) = curr {
        curr = &node.next;
        len += 1;
    }

    len
}

fn nth_node(node: &mut Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if n == 1 {
        return node.as_mut().unwrap().next.take();
    }

    nth_node(&mut node.as_mut().unwrap().next, n - 1)
}

fn append_to_tail(head: &mut Option<Box<ListNode>>, tail: Option<Box<ListNode>>) {
    let node = head.as_mut().unwrap();

    match node.next {
        None => {
            node.next = tail;
        }

        Some(_) => {
            append_to_tail(&mut node.next, tail);
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
    fn new(val: i32) -> Self {
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
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let mut a3 = ListNode::new(3);
        let mut a4 = ListNode::new(4);
        let a5 = ListNode::new(5);

        a4.next = Some(Box::new(a5));
        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let rot = rotate_right(Some(Box::new(a1.clone())), 1);
        assert_eq!(traverse(&rot), [5, 1, 2, 3, 4]);

        let rot = rotate_right(Some(Box::new(a1.clone())), 2);
        assert_eq!(traverse(&rot), [4, 5, 1, 2, 3]);

        let rot = rotate_right(Some(Box::new(a1.clone())), 3);
        assert_eq!(traverse(&rot), [3, 4, 5, 1, 2]);

        let rot = rotate_right(Some(Box::new(a1.clone())), 4);
        assert_eq!(traverse(&rot), [2, 3, 4, 5, 1]);

        let rot = rotate_right(Some(Box::new(a1.clone())), 5);
        assert_eq!(traverse(&rot), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_edge_cases() {
        let mut a1 = ListNode::new(0);

        let rot = rotate_right(Some(Box::new(a1.clone())), 0);
        assert_eq!(traverse(&rot), [0]);

        let rot = rotate_right(Some(Box::new(a1.clone())), 1);
        assert_eq!(traverse(&rot), [0]);

        let rot = rotate_right(Some(Box::new(a1.clone())), 100);
        assert_eq!(traverse(&rot), [0]);

        let a2 = ListNode::new(1);
        a1.next = Some(Box::new(a2));

        let rot = rotate_right(Some(Box::new(a1.clone())), 99);
        assert_eq!(traverse(&rot), [1, 0]);

        let rot = rotate_right(Some(Box::new(a1.clone())), 100);
        assert_eq!(traverse(&rot), [0, 1]);
    }
}

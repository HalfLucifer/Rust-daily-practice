pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if length(&head) < k {
        return head;
    }

    let mut node = head;
    let next = nth_node(&mut node, k);
    let mut reversed = reverse(&mut node, k);

    append_to_tail(&mut reversed, reverse_k_group(next, k));

    reversed
}

pub fn reverse_k_group_with_array(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut ret = None;
    let mut tail = &mut ret;
    let mut data = vec![];

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = None;
        data.push(node);

        if data.len() == k as usize {
            data.reverse();

            for p in data {
                tail = &mut tail.insert(p).next;
            }

            data = vec![];
        }
    }

    if data.len() > 0 as usize {
        for p in data {
            tail = &mut tail.insert(p).next;
        }
    }

    ret
}

fn append_to_tail(head: &mut Option<Box<ListNode>>, tail: Option<Box<ListNode>>) {
    let mut node = head.as_mut().unwrap();

    match node.next {
        Some(_) => {
            append_to_tail(&mut node.next, tail);
        }

        None => {
            node.next = tail;
        }
    }
}

fn length(head: &Option<Box<ListNode>>) -> i32 {
    let mut curr = head;
    let mut len = 0;

    while let Some(node) = curr {
        curr = &node.next;
        len += 1;
    }

    len
}

fn reverse(head: &mut Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head.take();
    let mut count = k;

    while count > 0 {
        if let Some(mut node) = curr {
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        count -= 1;
    }

    prev.take()
}

fn nth_node(node: &mut Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if n == 1 {
        return node.as_mut().unwrap().next.take();
    }

    nth_node(&mut node.as_mut().unwrap().next, n - 1)
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

        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 1)), [1, 2, 3, 4, 5]);
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 2)), [2, 1, 4, 3, 5]);
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 3)), [3, 2, 1, 4, 5]);
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 4)), [4, 3, 2, 1, 5]);
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 5)), [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_edge_cases() {
        let mut a1 = ListNode::new(0);
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 1)), [0]);
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 100)), [0]);

        let a2 = ListNode::new(1);
        a1.next = Some(Box::new(a2));
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 2)), [1, 0]);
        assert_eq!(traverse(&reverse_k_group(Some(Box::new(a1.clone())), 100)), [0, 1]);
    }
}

/*
    Given a linked list, swap every two adjacent nodes and return its head
*/
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut m) = head {
        match m.next {
            Some(mut n) => {
                m.next = swap_pairs(n.next);
                n.next = Some(m);
                return Some(n);
            }

            None => {
                return Some(m);
            }
        }
    }

    None
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
        let a4 = ListNode::new(4);

        a3.next = Some(Box::new(a4));
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        assert_eq!(
            traverse(swap_pairs(Some(Box::new(a1.clone())))),
            [2, 1, 4, 3]
        );
    }

    #[test]
    fn test_odd_nodes_cases() {
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
            traverse(swap_pairs(Some(Box::new(a1.clone())))),
            [2, 1, 4, 3, 5]
        );
    }

    #[test]
    fn test_edge_cases() {
        let mut a1 = ListNode::new(0);
        assert_eq!(traverse(swap_pairs(Some(Box::new(a1.clone())))), [0]);

        let a2 = ListNode::new(1);
        a1.next = Some(Box::new(a2));
        assert_eq!(traverse(swap_pairs(Some(Box::new(a1.clone())))), [1, 0]);
    }
}

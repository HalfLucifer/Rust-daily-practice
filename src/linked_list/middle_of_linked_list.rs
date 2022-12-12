pub fn get_middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        slow = &slow.as_ref().unwrap().next;
    }

    slow.clone()
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

        a4.next = Some(Box::new(a5.clone()));
        a3.next = Some(Box::new(a4.clone()));
        a2.next = Some(Box::new(a3.clone()));
        a1.next = Some(Box::new(a2.clone()));

        assert_eq!(get_middle_node(Some(Box::new(a1))), Some(Box::new(a3)));
    }

    #[test]
    fn test_even_nodes_cases() {
        let mut a1 = ListNode::new(1);
        let mut a2 = ListNode::new(2);
        let mut a3 = ListNode::new(3);
        let a4 = ListNode::new(4);

        a3.next = Some(Box::new(a4.clone()));
        a2.next = Some(Box::new(a3.clone()));
        a1.next = Some(Box::new(a2.clone()));

        assert_eq!(get_middle_node(Some(Box::new(a1))), Some(Box::new(a3)));
    }

    #[test]
    fn test_edge_cases() {
        let mut a1 = ListNode::new(0);
        assert_eq!(
            get_middle_node(Some(Box::new(a1.clone()))),
            Some(Box::new(a1.clone()))
        );

        let a2 = ListNode::new(1);
        a1.next = Some(Box::new(a2.clone()));
        assert_eq!(
            get_middle_node(Some(Box::new(a1.clone()))),
            Some(Box::new(a2.clone()))
        );
    }
}

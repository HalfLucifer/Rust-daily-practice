/*
   Given head, the head of a linked list, determine if the linked list has a cycle in it.
   Return true if there is a cycle in the linked list. Otherwise, return false.
*/
use std::cell::RefCell;
use std::rc::Rc;

pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    if let Some(node) = head {
        let mut slow = Some(Rc::clone(&node));
        let mut fast = Some(Rc::clone(&node));

        while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some() {
            fast = traverse(fast, 2); // Move 2 steps forward
            slow = traverse(slow, 1); // Move 1 step forward

            if fast == slow {
                // There is a cycle if slow meets fast
                return true;
            }
        }
    }

    false
}

fn traverse(head: Option<Rc<RefCell<ListNode>>>, index: usize) -> Option<Rc<RefCell<ListNode>>> {
    if index == 0 {
        return head;
    }

    if let Some(node) = head {
        if let Some(next) = &node.borrow().next {
            return traverse(Some(Rc::clone(next)), index - 1);
        }
    }

    None
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
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
    fn test_cycle_cases() {
        let a1 = Rc::new(RefCell::new(ListNode::new(1)));
        let a2 = Rc::new(RefCell::new(ListNode::new(2)));
        let a3 = Rc::new(RefCell::new(ListNode::new(3)));
        let a4 = Rc::new(RefCell::new(ListNode::new(4)));
        let a5 = Rc::new(RefCell::new(ListNode::new(5)));

        a1.borrow_mut().next = Some(Rc::clone(&a2));
        a2.borrow_mut().next = Some(Rc::clone(&a3));
        a3.borrow_mut().next = Some(Rc::clone(&a4));
        a4.borrow_mut().next = Some(Rc::clone(&a5));
        a5.borrow_mut().next = Some(Rc::clone(&a3));

        assert_eq!(has_cycle(Some(a1.clone())), true);
    }

    #[test]
    fn test_no_cycle_cases() {
        let a1 = Rc::new(RefCell::new(ListNode::new(1)));
        let a2 = Rc::new(RefCell::new(ListNode::new(2)));
        let a3 = Rc::new(RefCell::new(ListNode::new(3)));
        let a4 = Rc::new(RefCell::new(ListNode::new(4)));
        let a5 = Rc::new(RefCell::new(ListNode::new(5)));

        a1.borrow_mut().next = Some(Rc::clone(&a2));
        a2.borrow_mut().next = Some(Rc::clone(&a3));
        a3.borrow_mut().next = Some(Rc::clone(&a4));
        a4.borrow_mut().next = Some(Rc::clone(&a5));

        assert_eq!(has_cycle(Some(a1.clone())), false);
    }

    #[test]
    fn test_edge_cases() {
        let a1 = Rc::new(RefCell::new(ListNode::new(0)));
        assert_eq!(has_cycle(Some(a1.clone())), false);
    }
}

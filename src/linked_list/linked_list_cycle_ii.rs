/*
   Given the head of a linked list, return the node where the cycle begins. If there is no
   cycle, return null.
*/
use std::cell::RefCell;
use std::rc::Rc;

pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if let Some(node) = head {
        let mut slow = Some(Rc::clone(&node));
        let mut fast = Some(Rc::clone(&node));

        while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some() {
            fast = traverse(fast, 2); // Move 2 steps forward
            slow = traverse(slow, 1); // Move 1 step forward

            // There is a cycle if slow meets fast
            if fast == slow {
                // slow is moving from the head
                // fast is moving from the meeting node
                slow = Some(Rc::clone(&node));

                // Traverse until slow meets fast
                while fast != slow {
                    fast = traverse(fast, 1);
                    slow = traverse(slow, 1);
                }

                // Found the cycle starting node when slow meets fast
                return slow;
            }
        }
    }

    None
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

        let res = detect_cycle(Some(a1.clone()));
        assert_eq!(res.unwrap().borrow().val, 3);
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

        assert_eq!(detect_cycle(Some(a1.clone())), None);
    }

    #[test]
    fn test_edge_cases() {
        let a1 = Rc::new(RefCell::new(ListNode::new(0)));
        assert_eq!(detect_cycle(Some(a1.clone())), None);
    }
}

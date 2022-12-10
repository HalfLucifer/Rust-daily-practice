pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut curr1 = &l1;
    let mut curr2 = &l2;
    let mut dummy = ListNode::new(-1);
    let mut head = &mut dummy;
    let mut carry = 0;

    loop {
        match (curr1, curr2) {
            (Some(_), Some(_)) => {
                let sum = add_digit(
                    curr1.as_ref().unwrap().val,
                    curr2.as_ref().unwrap().val,
                    &mut carry,
                );
                head.next = Some(Box::new(ListNode::new(sum)));
                head = head.next.as_mut().unwrap();
                curr1 = &curr1.as_ref().unwrap().next;
                curr2 = &curr2.as_ref().unwrap().next;
            }

            (Some(_), None) => {
                while curr1.is_some() {
                    let sum = add_digit(curr1.as_ref().unwrap().val, 0, &mut carry);
                    head.next = Some(Box::new(ListNode::new(sum)));
                    head = head.next.as_mut().unwrap();
                    curr1 = &curr1.as_ref().unwrap().next;
                }
            }

            (None, Some(_)) => {
                while curr2.is_some() {
                    let sum = add_digit(0, curr2.as_ref().unwrap().val, &mut carry);
                    head.next = Some(Box::new(ListNode::new(sum)));
                    head = head.next.as_mut().unwrap();
                    curr2 = &curr2.as_ref().unwrap().next;
                }
            }

            _ => {
                break;
            }
        }
    }

    if carry == 1 {
        head.next = Some(Box::new(ListNode::new(1)));
    }

    dummy.next
}

fn add_digit(num1: i32, num2: i32, carry: &mut i32) -> i32 {
    let mut sum = num1 + num2 + *carry;

    if sum > 9 {
        *carry = 1;
        sum -= 10;
    } else {
        *carry = 0;
    }

    sum
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
        let mut a1 = ListNode::new(2);
        let mut a2 = ListNode::new(4);
        let a3 = ListNode::new(3);
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));

        let mut b1 = ListNode::new(5);
        let mut b2 = ListNode::new(6);
        let b3 = ListNode::new(4);
        b2.next = Some(Box::new(b3));
        b1.next = Some(Box::new(b2));

        assert_eq!(
            traverse(add_two_numbers(Some(Box::new(a1)), Some(Box::new(b1)))),
            [7, 0, 8]
        );
    }

    #[test]
    fn test_carry_cases() {
        let mut a1 = ListNode::new(9);
        let a2 = ListNode::new(9);
        a1.next = Some(Box::new(a2));

        let mut b1 = ListNode::new(9);
        let mut b2 = ListNode::new(9);
        let mut b3 = ListNode::new(9);
        let mut b4 = ListNode::new(9);
        let b5 = ListNode::new(9);
        b4.next = Some(Box::new(b5));
        b3.next = Some(Box::new(b4));
        b2.next = Some(Box::new(b3));
        b1.next = Some(Box::new(b2));

        assert_eq!(
            traverse(add_two_numbers(Some(Box::new(a1)), Some(Box::new(b1)))),
            [8, 9, 0, 0, 0, 1]
        );

        let mut a1 = ListNode::new(9);
        let mut a2 = ListNode::new(9);
        let a3 = ListNode::new(1);
        a2.next = Some(Box::new(a3));
        a1.next = Some(Box::new(a2));
        let b1 = ListNode::new(1);
        
        assert_eq!(
            traverse(add_two_numbers(Some(Box::new(a1)), Some(Box::new(b1)))),
            [0, 0, 2]
        );
    }

    #[test]
    fn test_edge_cases() {
        let a1 = ListNode::new(0);
        let b1 = ListNode::new(0);
        assert_eq!(
            traverse(add_two_numbers(Some(Box::new(a1)), Some(Box::new(b1)))),
            [0]
        );

        let a1 = ListNode::new(1);
        let b1 = ListNode::new(9);
        assert_eq!(
            traverse(add_two_numbers(Some(Box::new(a1)), Some(Box::new(b1)))),
            [0, 1]
        );
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub struct SinglyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    meet: Option<Rc<RefCell<Node>>>,
    length: usize,
}

struct Node {
    elem: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            meet: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, elem: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
        }));

        match self.head.take() {
            Some(node) => {
                new_node.borrow_mut().next = Some(Rc::clone(&node));
                self.head = Some(Rc::clone(&new_node));
            }

            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }

        self.length += 1;
    }

    pub fn traverse(&self) -> Vec<i32> {
        fn traverse_recursive(curr: &Option<Rc<RefCell<Node>>>, result: &mut Vec<i32>) {
            if let Some(node) = curr {
                result.push(node.borrow().elem);
                traverse_recursive(&node.borrow().next, result);
            }
        }

        let mut result = vec![];
        traverse_recursive(&self.head, &mut result);
        result
    }

    pub fn traverse_iteratively(&self) -> Vec<i32> {
        let mut result = vec![];

        let mut curr = None;
        if let Some(head) = &self.head {
            curr = Some(Rc::clone(&head));
        }

        while curr.is_some() {
            if let Some(node) = curr.as_ref() {
                let cloned = Rc::clone(&node);
                result.push(cloned.borrow().elem);

                if let Some(next) = &cloned.borrow().next {
                    curr = Some(Rc::clone(next));
                } else {
                    curr = None;
                };
            }
        }

        result
    }

    pub fn link_tail_to_node(&mut self, target_index: usize) {
        assert!(target_index > 0);

        let tail = Self::traverse_node(&self.head, self.length - 1);
        let target = Self::traverse_node(&self.head, target_index - 1);

        if let Some(node) = &tail {
            node.borrow_mut().next = target;
        }
    }

    pub fn has_cycle(&mut self) -> bool {
        self.find_meeting_node()
    }

    pub fn find_cycle(&mut self) -> Option<i32> {
        self.find_meeting_node();

        if self.meet.is_none() {
            return None;
        }

        let mut turtle = None;
        if let Some(head) = &self.head {
            turtle = Some(Rc::clone(&head));
        }

        loop {
            if turtle.as_ref().unwrap().borrow().elem == self.meet.as_ref().unwrap().borrow().elem {
                break;
            }

            turtle = Self::traverse_node(&turtle, 1);
            self.meet = Self::traverse_node(&self.meet, 1);
        }

        if let Some(node) = turtle {
            return Some(node.borrow().elem);
        }

        None
    }

    fn find_meeting_node(&mut self) -> bool {
        let mut turtle = None;
        let mut rabbit = None;

        if let Some(head) = &self.head {
            turtle = Some(Rc::clone(&head));
            rabbit = Some(Rc::clone(&head));
        }

        while rabbit.is_some() {
            turtle = Self::traverse_node(&turtle, 1);
            rabbit = Self::traverse_node(&rabbit, 2);

            if rabbit.is_none() {
                break;
            }

            if turtle.as_ref().unwrap().borrow().elem == rabbit.as_ref().unwrap().borrow().elem {
                self.meet = turtle;
                return true;
            }
        }

        false
    }

    fn traverse_node(head: &Option<Rc<RefCell<Node>>>, index: usize) -> Option<Rc<RefCell<Node>>> {
        if let Some(head) = &head {
            return SinglyLinkedList::traverse_node_recursive(Some(Rc::clone(head)), index);
        }

        None
    }

    fn traverse_node_recursive(
        curr: Option<Rc<RefCell<Node>>>,
        index: usize,
    ) -> Option<Rc<RefCell<Node>>> {
        if index == 0 {
            return curr;
        } else {
            if let Some(node) = curr {
                if let Some(next) = &node.borrow().next {
                    return SinglyLinkedList::traverse_node_recursive(
                        Some(Rc::clone(next)),
                        index - 1,
                    );
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        for cycle in 2..10 {
            let mut s = SinglyLinkedList::new();

            for i in (0..10).rev() {
                s.push_front(i);
            }

            assert_eq!(s.has_cycle(), false);

            s.link_tail_to_node(cycle);

            assert_eq!(s.has_cycle(), true);
            assert_eq!(s.find_cycle(), Some(cycle as i32 - 1));
        }
    }

    #[test]
    fn test_edge_cases() {
        let mut s = SinglyLinkedList::new();

        s.push_front(0);
        assert_eq!(s.has_cycle(), false);
        assert_eq!(s.find_cycle(), None);

        s.push_front(1);
        s.link_tail_to_node(1);
        assert_eq!(s.has_cycle(), true);
        assert_eq!(s.find_cycle(), Some(1));

        s.push_front(2);
        s.link_tail_to_node(1);
        assert_eq!(s.has_cycle(), true);
        assert_eq!(s.find_cycle(), Some(2));
    }

    #[test]
    fn test_traversal_cases() {
        let mut s = SinglyLinkedList::new();
        assert_eq!(s.traverse(), []);
        assert_eq!(s.traverse_iteratively(), []);

        s.push_front(0);
        assert_eq!(s.traverse(), [0]);
        assert_eq!(s.traverse_iteratively(), [0]);

        let mut s = SinglyLinkedList::new();
        for i in (0..10).rev() {
            s.push_front(i);
        }

        assert_eq!(s.traverse(), (0..10).collect::<Vec<_>>());
        assert_eq!(s.traverse_iteratively(), (0..10).collect::<Vec<_>>());

        assert_eq!(s.has_cycle(), false);
        assert_eq!(s.find_cycle(), None);
    }

    #[test]
    fn test_complete_cycle_cases() {
        let mut s = SinglyLinkedList::new();

        for i in (0..10).rev() {
            s.push_front(i);
        }

        s.link_tail_to_node(1);

        assert_eq!(s.has_cycle(), true);
        assert_eq!(s.find_cycle(), Some(0));
    }
}

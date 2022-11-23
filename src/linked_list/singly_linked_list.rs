pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.head = old_head.next;
        Some(old_head.elem)
    }

    pub fn insert(&mut self, index: usize, elem: T) -> Result<(), ()> {
        let mut pos = index;
        let mut curr = &mut self.head;

        // Traverse to target node
        while pos > 0 {
            if let Some(node) = curr.as_mut() {
                curr = &mut node.next;
            } else {
                return Err(());
            }

            pos -= 1;
        }

        // Take the target node out
        if let Some(mut node) = curr.take() {
            node.next = Some(Box::new(Node {
                elem,
                next: node.next,
            }));

            // Put the target node back
            *curr = Some(node);
        } else {
            return Err(());
        }

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let mut pos = index;
        let mut curr = &mut self.head;

        // Traverse to target node
        while pos > 0 {
            curr = &mut curr.as_mut()?.next;
            pos -= 1;
        }

        let node = curr.take()?;
        *curr = node.next;

        Some(node.elem)
    }

    pub fn traverse(&self) -> Vec<T> {
        let mut arr = vec![];
        let mut curr = &self.head;

        loop {
            if let Some(node) = curr.as_ref() {
                arr.push(node.elem);
                curr = &node.next;
            } else {
                break;
            }
        }

        arr
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut curr = &self.head;

        loop {
            if let Some(node) = curr.as_ref() {
                len += 1;
                curr = &node.next;
            } else {
                break;
            }
        }

        len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop_cases() {
        let mut s: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert!(s.is_empty());

        for i in 1..100 {
            s.push_front(i);
        }

        assert_eq!(s.len(), 99);

        for i in (1..100).rev() {
            assert_eq!(s.pop_front().unwrap(), i);
        }

        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }

    #[test]
    fn test_insert_cases() {
        let mut s: SinglyLinkedList<i32> = SinglyLinkedList::new();
        for i in (1..=5).rev() {
            s.push_front(i);
        }

        assert_eq!(s.insert(0, 10), Ok(()));
        assert_eq!(s.insert(2, 20), Ok(()));
        assert_eq!(s.insert(6, 30), Ok(()));
        assert_eq!(s.insert(8, 99), Err(()));
        assert_eq!(s.len(), 8);
        assert_eq!(s.traverse(), [1, 10, 2, 20, 3, 4, 5, 30]);
    }

    #[test]
    fn test_remove_cases() {
        let mut s: SinglyLinkedList<i32> = SinglyLinkedList::new();
        for i in (1..=5).rev() {
            s.push_front(i);
        }

        assert_eq!(s.remove(0), Some(1));
        assert_eq!(s.traverse(), [2, 3, 4, 5]);
        assert_eq!(s.remove(3), Some(5));
        assert_eq!(s.traverse(), [2, 3, 4]);

        assert_eq!(s.remove(3), None);
        assert_eq!(s.remove(2), Some(4));
        assert_eq!(s.remove(1), Some(3));
        assert_eq!(s.remove(0), Some(2));
        assert_eq!(s.remove(0), None);
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_traverse_cases() {
        let mut s: SinglyLinkedList<i32> = SinglyLinkedList::new();
        for i in (1..100).rev() {
            s.push_front(i);
        }

        assert_eq!(s.traverse(), (1..100).collect::<Vec<_>>());
        assert_eq!(s.len(), 99);
    }
}

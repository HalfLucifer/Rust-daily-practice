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
        let next = self.head.take();
        self.head = Some(Box::new(Node { elem, next }));
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.head.take();

        while let Some(mut node) = curr {
            let next = node.next;
            node.next = prev;

            prev = Some(node);
            curr = next;
        }

        self.head = prev.take();
    }

    pub fn traverse(&self) -> Vec<T> {
        let mut arr = vec![];
        let mut curr = &self.head;

        loop {
            match curr.as_ref() {
                Some(node) => {
                    arr.push(node.elem);
                    curr = &node.next;
                }
                None => break,
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut s: SinglyLinkedList<i32> = SinglyLinkedList::new();

        for i in 1..10 {
            s.push_front(i);
        }
        s.reverse();
        assert_eq!(s.traverse(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_edge_cases() {
        let mut s: SinglyLinkedList<i32> = SinglyLinkedList::new();
        s.reverse();
        assert_eq!(s.traverse(), []);
        s.push_front(0);
        assert_eq!(s.traverse(), [0]);
    }

    #[test]
    fn test_zeros_and_ones_cases() {
        let mut s: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let mut v = vec![];

        for i in 1..100 {
            if i % 2 == 0 {
                s.push_front(0);
                v.push(0);
            } else {
                s.push_front(1);
                v.push(1);
            }
        }
        s.reverse();
        assert_eq!(s.traverse(), v);
    }
}

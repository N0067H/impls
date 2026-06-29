type Link<T> = Option<Box<Node<T>>>;

pub struct SinglyLinkedList<T> {
    pub head: Link<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    pub fn insert(&mut self, val: T, pos: usize) {
        let mut current = &mut self.head;

        for _ in 0..pos {
            current = match current {
                Some(node) => &mut node.next,
                None => panic!("out of range"),
            };
        }

        let new_node = Box::new(Node {
            val,
            next: current.take(),
        });

        *current = Some(new_node);
    }

    pub fn remove(&mut self, pos: usize) -> T {
        let mut current = &mut self.head;

        for _ in 0..pos {
            current = match current {
                Some(node) => &mut node.next,
                None => panic!("out of range"),
            };
        }

        let mut target = current.take().expect("out of range");
        *current = target.next.take();

        target.val
    }
}

pub struct Node<T> {
    pub val: T,
    pub next: Link<T>,
}

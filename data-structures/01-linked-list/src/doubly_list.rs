use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub prev: WeakLink<T>,
    pub next: Link<T>,
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    pub head: Link<T>,
    pub tail: Link<T>,
    pub len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            prev: self.tail.as_ref().map(Rc::downgrade),
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
    }
}

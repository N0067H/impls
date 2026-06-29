use crate::{doubly_list::DoublyLinkedList, singly_list::SinglyLinkedList};

mod doubly_list;
mod singly_list;

fn main() {
    let mut singly = SinglyLinkedList::new();

    singly.insert(10, 0);
    singly.insert(20, 1);
    singly.insert(30, 2);
    singly.insert(15, 1);

    println!("removed from singly: {}", singly.remove(2));

    let mut doubly = DoublyLinkedList::new();

    doubly.push_back(10);
    doubly.push_back(20);
    doubly.push_back(30);
    doubly.push_front(5);

    println!("{:#?}", doubly);
}

// main.rs

pub mod dll;

use dll::DoublyLinkedList;

fn main() {
    let mut list = DoublyLinkedList::new();

    // Push some elements
    list.push_back("World");
    list.push_front("Hello");
    list.push_back("!");

    println!("Current list length: {}", list.len()); // Should be 3

    // Pop elements and print them
    println!("Popped from front: {:?}", list.pop_front()); // "Hello"
    println!("Popped from back: {:?}", list.pop_back()); // "!"
    println!("Popped from front: {:?}", list.pop_front()); // "World"
    println!("Popped from front: {:?}", list.pop_front()); // None

    println!("Final list length: {}", list.len()); // Should be 0
}


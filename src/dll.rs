
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Type aliases to make the code more readable
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

/// Internal Node structure for the list
#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
    prev: WeakLink<T>, // Use Weak to prevent reference cycles
}

/// The Doubly Linked List
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> Node<T> {
    /// Creates a new node wrapped in Rc and RefCell
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }))
    }
}

impl<T> DoublyLinkedList<T> {
    /// Creates a new, empty doubly linked list.
    /// ```
    /// let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// ```
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Adds an element to the front of the list.
    pub fn push_front(&mut self, val: T) {
        let new_head = Node::new(val);

        match self.head.take() {
            // List was not empty
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            // List was empty
            None => {
                self.tail = Some(Rc::clone(&new_head));
                self.head = Some(new_head);
            }
        }
        self.len += 1;
    }

    /// Adds an element to the back of the list.
    pub fn push_back(&mut self, val: T) {
        let new_tail = Node::new(val);

        match self.tail.take() {
            // List was not empty
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_tail);
            }
            // List was empty
            None => {
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
        }
        self.len += 1;
    }

    /// Removes the first element and returns it, or `None` if the list is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                // List has more than one element
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                // List is now empty
                None => {
                    self.tail.take();
                }
            }
            self.len -= 1;
            // Unwraps the value from Rc<RefCell<Node<T>>>
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    /// Removes the last element and returns it, or `None` if the list is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail
                .borrow_mut()
                .prev
                .take()
                .and_then(|weak_prev| weak_prev.upgrade())
            {
                // List has more than one element
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                // List is now empty
                None => {
                    self.head.take();
                }
            }
            self.len -= 1;
            // Unwraps the value from Rc<RefCell<Node<T>>>
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }
}

// Implement Drop to prevent stack overflow on long lists
impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        // Pop all elements to ensure nodes are deallocated iteratively
        while self.pop_front().is_some() {}
    }
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_and_pop_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_and_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_mixed_push_pop() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2); // list: [2]
        list.push_back(3); // list: [2, 3]
        list.push_front(1); // list: [1, 2, 3]

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(1)); // list: [2, 3]
        assert_eq!(list.pop_back(), Some(3)); // list: [2]
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(2)); // list: []
        assert!(list.is_empty());
    }
}

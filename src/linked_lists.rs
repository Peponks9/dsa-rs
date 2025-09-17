// Single Linked List implementation in Rust

/// A node in the linked list
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
/// A singly linked list
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }
    // Get the length of the linked list
    pub fn len(&self) -> usize {
        self.length
    }
    // Check if linked list is empty 
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    // Insert at head
    pub fn insert_at_head(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }
    // Remove from head
    pub fn remove_from_head(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.length -= 1;
            Some(node.value)
        } else {
            None
        }
    }
    // insert at tail
}
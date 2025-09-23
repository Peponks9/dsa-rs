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
// Errors enum
#[derive(Debug)]
pub enum LinkedListError {
    IndexOutOfBounds,
    EmptyList,
    InvalidIndex,
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
    // insert at tail
    pub fn insert_at_tail(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        if let Some(mut current) = self.head.as_mut() {
            while let Some(ref mut next) = current.next {
                current = next;
            }
            current.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
        self.length += 1;
    }
    // insert at index
    pub fn insert_at_index(&mut self, index: usize, value: T) -> Result<(), LinkedListError> {
        if index > self.length {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        if index == 0 {
            self.insert_at_head(value);
            return Ok(());
        }
        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = node.next.as_mut();
            }
        }
        if let Some(node) = current {
            let new_node = Box::new(Node {
                value,
                next: node.next.take(),
            });
            node.next = Some(new_node);
            self.length += 1;
            Ok(())
        } else {
            Err(LinkedListError::InvalidIndex)
        }
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

    //remove from tail
    pub fn remove_from_tail(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        if self.head.as_ref().unwrap().next.is_none() {
            return self.remove_from_head();
        }
        let mut current = self.head.as_mut().unwrap();  
        while let Some(ref next) = current.next {  
            if next.next.is_none() {  
                let tail = current.next.take().unwrap();  
                self.length -= 1;
                return Some(tail.value);  
            }
            current = current.next.as_mut().unwrap();  
        }
        None  
    }
    // remove from index 
    pub fn remove_from_index(&mut self, index: usize) -> Result<T, LinkedListError> {
        if index >= self.length {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        if index == 0 {
            return self.remove_from_head().ok_or(LinkedListError::EmptyList);
        }
        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = node.next.as_mut();
            }
        }
        if let Some(node) = current {
            if let Some(mut to_remove) = node.next.take() {
                node.next = to_remove.next.take();
                self.length -= 1;
                return Ok(to_remove.value);
            }
        }
        Err(LinkedListError::InvalidIndex)

    }
    // reverse singly-linked list
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }
    // Get value at index
    pub fn get(&self, index: usize) -> Result<&T, LinkedListError> {
        if index >= self.length {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        let mut current = self.head.as_ref();
        for _ in 0..index {
            if let Some(node) = current {
                current = node.next.as_ref();
            }
        }
        if let Some(node) = current {
            Ok(&node.value)
        } else {
            Err(LinkedListError::InvalidIndex)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.insert_at_head(1);
        list.insert_at_tail(2);
        list.insert_at_index(1, 3).unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(0).unwrap(), &1);
        assert_eq!(list.get(1).unwrap(), &3);
        assert_eq!(list.get(2).unwrap(), &2);
        assert_eq!(list.remove_from_index(1).unwrap(), 3);
        assert_eq!(list.len(), 2);
        assert_eq!(list.remove_from_head().unwrap(), 1);
        assert_eq!(list.remove_from_tail().unwrap(), 2);
        assert!(list.is_empty());
        list.insert_at_head(4);
        list.insert_at_tail(5);
        list.reverse();
        assert_eq!(list.get(0).unwrap(), &5);
        assert_eq!(list.get(1).unwrap(), &4);
    }
}
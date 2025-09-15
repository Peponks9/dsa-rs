/// A dynamic array implementation using Vec
pub struct DynamicArray<T> {
    data: Vec<T>,
}

impl<T> DynamicArray<T> {
    /// Create a new empty dynamic array
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Create a new dynamic array with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Get the length of the dynamic array
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the dynamic array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the capacity of the dynamic array
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Push an element to the end
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Pop an element from the end
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Get a reference to the element at the given index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Get a mutable reference to the element at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Set the element at the given index
    pub fn set(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index < self.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    /// Insert an element at the given index
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index <= self.len() {
            self.data.insert(index, value);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    /// Remove an element at the given index
    pub fn remove(&mut self, index: usize) -> Result<T, &'static str> {
        if index < self.len() {
            Ok(self.data.remove(index))
        } else {
            Err("Index out of bounds")
        }
    }

    /// Clear the dynamic array
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Iterate over the dynamic array
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Iterate mutably over the dynamic array
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }

    /// Shrink the capacity to fit the current length
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Reserve additional capacity
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
}

impl<T: Clone> Clone for DynamicArray<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for DynamicArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicArray")
            .field("data", &self.data)
            .finish()
    }
}

impl<T> Default for DynamicArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr: DynamicArray<i32> = DynamicArray::new();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let arr: DynamicArray<i32> = DynamicArray::with_capacity(10);
        assert_eq!(arr.len(), 0);
        assert!(arr.capacity() >= 10);
    }

    #[test]
    fn test_push_and_pop() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.pop(), Some(3));
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.pop(), None);
    }

    #[test]
    fn test_get() {
        let mut arr = DynamicArray::new();
        arr.push(10);
        arr.push(20);
        assert_eq!(arr.get(0), Some(&10));
        assert_eq!(arr.get(1), Some(&20));
        assert_eq!(arr.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut arr = DynamicArray::new();
        arr.push(10);
        if let Some(val) = arr.get_mut(0) {
            *val = 42;
        }
        assert_eq!(arr.get(0), Some(&42));
    }

    #[test]
    fn test_set() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        assert!(arr.set(0, 42).is_ok());
        assert_eq!(arr.get(0), Some(&42));
        assert!(arr.set(2, 99).is_err());
    }

    #[test]
    fn test_insert() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(3);
        assert!(arr.insert(1, 2).is_ok());
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), Some(&3));
        assert!(arr.insert(5, 4).is_err());
    }

    #[test]
    fn test_remove() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.remove(1), Ok(2));
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&3));
        assert!(arr.remove(2).is_err());
    }

    #[test]
    fn test_clear() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.clear();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_iter() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        let mut iter = arr.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        for val in arr.iter_mut() {
            *val += 10;
        }
        assert_eq!(arr.get(0), Some(&11));
        assert_eq!(arr.get(1), Some(&12));
    }

    #[test]
    fn test_clone() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        let cloned = arr.clone();
        assert_eq!(cloned.len(), 2);
        assert_eq!(cloned.get(0), Some(&1));
        assert_eq!(cloned.get(1), Some(&2));
    }

    #[test]
    fn test_default() {
        let arr: DynamicArray<i32> = Default::default();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }
}
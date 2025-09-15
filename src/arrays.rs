/// A fixed-size array implementation
pub struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Array<T, N> {
    /// Create a new array from a given array
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        N
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        N == 0
    }

    /// Get a reference to the element at the given index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            Some(&self.data[index])
        } else {
            None
        }
    }

    /// Get a mutable reference to the element at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < N {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    /// Set the element at the given index
    pub fn set(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index < N {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    /// Iterate over the array
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Iterate mutably over the array
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    /// Create a new array filled with default values
    pub fn default() -> Self {
        Self { data: [T::default(); N] }
    }
}

impl<T: Clone, const N: usize> Clone for Array<T, N> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for Array<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array")
            .field("data", &self.data)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr: Array<i32, 3> = Array::new([1, 2, 3]);
        assert_eq!(arr.len(), 3);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_empty_array() {
        let arr: Array<i32, 0> = Array::new([]);
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_get() {
        let arr: Array<i32, 3> = Array::new([1, 2, 3]);
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), Some(&3));
        assert_eq!(arr.get(3), None);
    }

    #[test]
    fn test_get_mut() {
        let mut arr: Array<i32, 3> = Array::new([1, 2, 3]);
        if let Some(val) = arr.get_mut(1) {
            *val = 42;
        }
        assert_eq!(arr.get(1), Some(&42));
    }

    #[test]
    fn test_set() {
        let mut arr: Array<i32, 3> = Array::new([1, 2, 3]);
        assert!(arr.set(1, 42).is_ok());
        assert_eq!(arr.get(1), Some(&42));
        assert!(arr.set(3, 99).is_err());
    }

    #[test]
    fn test_iter() {
        let arr: Array<i32, 3> = Array::new([1, 2, 3]);
        let mut iter = arr.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut arr: Array<i32, 3> = Array::new([1, 2, 3]);
        for val in arr.iter_mut() {
            *val += 10;
        }
        assert_eq!(arr.get(0), Some(&11));
        assert_eq!(arr.get(1), Some(&12));
        assert_eq!(arr.get(2), Some(&13));
    }

    #[test]
    fn test_default() {
        let arr: Array<i32, 3> = Array::default();
        assert_eq!(arr.get(0), Some(&0));
        assert_eq!(arr.get(1), Some(&0));
        assert_eq!(arr.get(2), Some(&0));
    }

    #[test]
    fn test_clone() {
        let arr: Array<i32, 3> = Array::new([1, 2, 3]);
        let cloned = arr.clone();
        assert_eq!(cloned.get(0), Some(&1));
        assert_eq!(cloned.get(1), Some(&2));
        assert_eq!(cloned.get(2), Some(&3));
    }
}
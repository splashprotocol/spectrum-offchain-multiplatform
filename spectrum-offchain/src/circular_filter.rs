use circular_buffer::CircularBuffer;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub struct CircularFilter<const N: usize, T> {
    buffer: CircularBuffer<N, T>,
    filter: HashSet<T>,
}

impl<const N: usize, T> CircularFilter<N, T> {
    pub fn new() -> Self {
        Self {
            buffer: CircularBuffer::new(),
            filter: HashSet::new(),
        }
    }
}

impl<const N: usize, T: Hash + Eq + Clone> CircularFilter<N, T> {
    /// Adds new element to filter.
    /// Returns evicted element is filter is full.
    pub fn add(&mut self, a: T) -> Option<T> {
        if self.filter.insert(a.clone()) {
            if let Err(a) = self.buffer.try_push_back(a) {
                if let Some(evicted_element) = self.buffer.pop_front() {
                    self.filter.remove(&evicted_element);
                    self.buffer.push_back(a);
                    return Some(evicted_element);
                }
            }
        }
        None
    }

    pub fn remove(&mut self, a: &T) -> bool {
        self.filter.remove(a)
    }

    #[cfg(test)]
    fn contains(&self, a: &T) -> bool {
        self.filter.contains(a)
    }
}

#[cfg(test)]
mod tests {
    use crate::circular_filter::CircularFilter;

    #[test]
    fn add_check_rotate() {
        let mut f = CircularFilter::<3, usize>::new();
        f.add(1);
        assert!(f.contains(&1));
        f.add(2);
        assert!(f.contains(&2));
        f.add(3);
        assert!(f.contains(&3));
        f.add(4);
        assert!(f.contains(&4));
        assert!(!f.contains(&1));
        assert!(f.contains(&2));
    }
}
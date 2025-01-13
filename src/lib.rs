use std::collections::{BinaryHeap, HashMap};

pub const TOP_K_LIMIT: usize = 5;

pub trait Algorithm {
    fn load(data: HashMap<String, Vec<f32>>) -> Self;
    fn search(&self, query: &str) -> Option<Vec<&String>>;
}

pub struct LimitedHeap<T> {
    heap: BinaryHeap<T>,
    limit: usize,
}

impl<T: Ord> LimitedHeap<T> {
    pub fn new(limit: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(limit),
            limit,
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.heap.len() < self.limit {
            self.heap.push(value);
        } else if let Some(top) = self.heap.peek() {
            if &value < top {
                self.heap.pop();
                self.heap.push(value)
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_insertion() {
        let mut heap = LimitedHeap::new(3);

        heap.insert(10);
        heap.insert(20);
        heap.insert(5);
        heap.insert(1);
        heap.insert(15);

        let mut top_k: Vec<&i32> = heap.iter().collect();

        assert_eq!(top_k.len(), 3);

        top_k.sort_by(|a, b| b.cmp(a));

        assert_eq!(*top_k[0], 10);
        assert_eq!(*top_k[1], 5);
        assert_eq!(*top_k[2], 1);
    }
}

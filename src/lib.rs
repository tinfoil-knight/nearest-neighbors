use std::{collections::BinaryHeap, ops::Deref};

pub const TOP_K_LIMIT: usize = 5;

pub trait Algorithm {
    fn search(&self, query: &str) -> Option<Vec<String>>;
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

    pub fn push(&mut self, value: T) {
        if self.heap.len() < self.limit {
            self.heap.push(value);
        } else if let Some(top) = self.heap.peek() {
            if &value < top {
                self.heap.pop();
                self.heap.push(value)
            }
        }
    }
}

impl<T> Deref for LimitedHeap<T> {
    type Target = BinaryHeap<T>;

    fn deref(&self) -> &Self::Target {
        &self.heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_insertion() {
        let mut heap = LimitedHeap::new(3);

        heap.push(10);
        heap.push(20);
        heap.push(5);
        heap.push(1);
        heap.push(15);

        let mut top_k: Vec<&i32> = heap.iter().collect();

        assert_eq!(top_k.len(), 3);

        top_k.sort_by(|a, b| b.cmp(a));

        assert_eq!(*top_k[0], 10);
        assert_eq!(*top_k[1], 5);
        assert_eq!(*top_k[2], 1);
    }
}

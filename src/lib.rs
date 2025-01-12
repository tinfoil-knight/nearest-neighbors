use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub const TOP_K_LIMIT: usize = 10;

pub trait Algorithm {
    fn load(data: HashMap<String, Vec<f32>>) -> Self;
    fn search(&self, query: &str) -> Option<Vec<&String>>;
}

pub struct KMinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
    limit: usize,
}

impl<T: Ord> KMinHeap<T> {
    pub fn new(k: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(k),
            limit: k,
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.heap.len() < self.limit {
            self.heap.push(Reverse(value));
        } else if let Some(Reverse(top)) = self.heap.peek() {
            if &value > top {
                self.heap.pop();
                self.heap.push(Reverse(value))
            }
        }
    }

    pub fn get_top_k(&self) -> Vec<&T> {
        let mut result: Vec<&T> = self.heap.iter().map(|Reverse(v)| v).collect();
        result.sort_by(|a, b| b.cmp(a));
        result
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
    fn test_k_min_heap_insertion() {
        let mut k_min_heap = KMinHeap::new(3);

        k_min_heap.insert(10);
        k_min_heap.insert(20);
        k_min_heap.insert(5);
        k_min_heap.insert(1);
        k_min_heap.insert(15);

        let top_k = k_min_heap.get_top_k();

        assert_eq!(top_k.len(), 3);

        assert_eq!(*top_k[0], 20);
        assert_eq!(*top_k[1], 15);
        assert_eq!(*top_k[2], 10);
    }
}

use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{Deref, DerefMut},
};

pub mod exact;
pub mod kdtree;
pub mod lsh;
pub mod vptree;

use exact::Exact;
use kdtree::KDTree;
use lsh::LSH;
use vptree::VPTree;

pub trait Algorithm {
    fn search(&self, query: &str, k: usize) -> Option<Vec<String>>;
}

pub fn get_search_algorithm<'a>(
    flag: &str,
    data: &'a HashMap<String, Vec<f32>>,
) -> Box<dyn Algorithm + 'a> {
    match flag {
        "exact" => Box::new(Exact::load(data)),
        "kdtree" => Box::new(KDTree::load(data)),
        "vptree" => Box::new(VPTree::load(data)),
        "lsh" => Box::new(LSH::load(data)),
        _ => Box::new(Exact::load(data)),
    }
}

pub fn load_dataset(path: &str) -> io::Result<HashMap<String, Vec<f32>>> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut word_map = HashMap::new();
    let mut dimensions = 0;

    for line in reader.lines() {
        let line = line?;

        let Some((word, vector_s)) = line.split_once(' ') else {
            panic!("invalid line");
        };

        let vector: Vec<f32> = vector_s
            .split_terminator(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        if dimensions == 0 {
            dimensions = vector.len();
        }

        assert_eq!(dimensions, vector.len());

        word_map.insert(word.to_string(), vector);
    }

    Ok(word_map)
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

impl<T> DerefMut for LimitedHeap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap
    }
}

#[derive(Debug)]
pub struct BinaryTree<T>(pub Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

impl<T> BinaryTree<T> {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        Self::len_helper(self)
    }

    fn len_helper(tree: &BinaryTree<T>) -> usize {
        tree.0.as_ref().map_or(0, |node| {
            1 + Self::len_helper(&node.left) + Self::len_helper(&node.right)
        })
    }
}

pub fn distance(x: &[f32], y: &[f32]) -> f32 {
    x.iter()
        .zip(y.iter())
        .map(|(a, b)| f32::powi(a - b, 2))
        .sum::<f32>()
        .sqrt()
}

pub fn dot_product(x: &[f32], y: &[f32]) -> f32 {
    x.iter().zip(y.iter()).map(|(a, b)| a * b).sum()
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

        assert_eq!(heap.iter().collect::<Vec<&i32>>().len(), 3);

        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(1));
    }
}

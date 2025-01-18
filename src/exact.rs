use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
};

use crate::{dot_product, Algorithm, LimitedHeap};

pub struct Exact {
    data: HashMap<String, Vec<f32>>,
}

impl Algorithm for Exact {
    fn search(&self, query: &str, k: usize) -> Option<Vec<String>> {
        if let Some(a) = self.data.get(query) {
            let a_mag = a.iter().map(|x| x * x).sum::<f32>().sqrt();
            let mut k_min_heap: LimitedHeap<Reverse<HeapItem>> = LimitedHeap::new(k);

            for (key, b) in self.data.iter() {
                let b_mag = b.iter().map(|x| x * x).sum::<f32>().sqrt();
                let cosine_similarity: f32 = if a_mag == 0.0 || b_mag == 0.0 {
                    0.0
                } else {
                    let dot_product: f32 = dot_product(a, b);
                    dot_product / (a_mag * b_mag)
                };

                k_min_heap.push(Reverse(HeapItem(cosine_similarity, key)));
            }

            if k_min_heap.is_empty() {
                None
            } else {
                let mut v: Vec<&HeapItem> = k_min_heap.iter().map(|Reverse(item)| item).collect();
                v.sort();
                Some(v.iter().rev().map(|x| x.1.to_owned()).collect())
            }
        } else {
            None
        }
    }
}

impl Exact {
    pub fn load(data: &HashMap<String, Vec<f32>>) -> Self {
        Self { data: data.clone() }
    }
}

#[derive(Debug)]
struct HeapItem<'a>(f32, &'a str);

impl PartialEq for HeapItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HeapItem<'_> {}

impl PartialOrd for HeapItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // ignoring NaN for f32
        self.0.partial_cmp(&other.0).unwrap()
    }
}

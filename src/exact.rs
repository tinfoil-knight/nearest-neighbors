use std::cmp::Ordering;

use crate::{dot_product, Algorithm, LimitedHeap, VectorID};

pub struct Exact {
    data: Vec<(VectorID, Vec<f32>)>,
}

impl Algorithm for Exact {
    fn search(&self, query: &[f32], k: usize) -> Vec<VectorID> {
        let a_mag = query.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mut k_min_heap: LimitedHeap<HeapItem> = LimitedHeap::new(k);

        for (key, b) in &self.data {
            let b_mag = b.iter().map(|x| x * x).sum::<f32>().sqrt();
            let cosine_similarity: f32 = if a_mag == 0.0 || b_mag == 0.0 {
                0.0
            } else {
                let dot_product: f32 = dot_product(query, b);
                dot_product / (a_mag * b_mag)
            };

            k_min_heap.push(HeapItem(-1.0 * cosine_similarity, *key));
        }

        let mut v: Vec<&HeapItem> = k_min_heap.iter().collect();
        v.sort();
        v.into_iter().map(|&HeapItem(_, id)| id).collect()
    }
}

impl Exact {
    pub fn load(data: &[(VectorID, Vec<f32>)]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

#[derive(Debug)]
struct HeapItem(f32, VectorID);

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HeapItem {}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // ignoring NaN for f32
        self.0.partial_cmp(&other.0).unwrap()
    }
}

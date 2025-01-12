use std::{cmp::Ordering, collections::HashMap};

use early_ann::{Algorithm, KMinHeap, TOP_K_LIMIT};

pub struct Exact {
    data: HashMap<String, Vec<f32>>,
}

impl Algorithm for Exact {
    fn load(data: HashMap<String, Vec<f32>>) -> Self {
        Self { data }
    }

    fn search(&self, query: &str) -> Option<Vec<&String>> {
        if let Some(a) = self.data.get(query) {
            let a_mag = a.iter().map(|x| x * x).sum::<f32>();
            let mut k_min_heap: KMinHeap<HeapItem> = KMinHeap::new(TOP_K_LIMIT);

            for (key, b) in self.data.iter() {
                let b_mag = b.iter().map(|x| x * x).sum::<f32>();
                // We're not taking the sqrt of the magnitude since that's more efficient.
                // That's why this is a reduced metric. Relative rankings are preserved.
                let reduced_cosine_similarity: f32 = if a_mag == 0.0 || b_mag == 0.0 {
                    0.0
                } else {
                    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
                    dot_product / (a_mag * b_mag)
                };

                k_min_heap.insert(HeapItem(reduced_cosine_similarity, key));
            }

            if k_min_heap.is_empty() {
                None
            } else {
                Some(k_min_heap.get_top_k().iter().map(|v| v.1).collect())
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct HeapItem<'a>(f32, &'a String);

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

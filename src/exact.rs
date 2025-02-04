use crate::{cosine_similarity, Algorithm, LimitedHeap, OrdItem, VectorID};

pub struct Exact {
    data: Vec<(VectorID, Vec<f32>)>,
}

impl Algorithm for Exact {
    fn search(&self, query: &[f32], k: usize) -> Vec<VectorID> {
        let mut k_min_heap: LimitedHeap<OrdItem<VectorID>> = LimitedHeap::new(k);

        for (key, b) in &self.data {
            let cosine_similarity: f32 = cosine_similarity(query, b);
            k_min_heap.push(OrdItem(-1.0 * cosine_similarity, *key));
        }

        let mut v: Vec<&OrdItem<VectorID>> = k_min_heap.iter().collect();
        v.sort();
        v.into_iter().map(|&OrdItem(_, id)| id).collect()
    }
}

impl Exact {
    pub fn load(data: &[(VectorID, Vec<f32>)]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use rand::Rng;

use crate::{dot_product, Algorithm};

type Hash = Vec<u8>;
type PlaneNorm = Vec<f32>;

pub struct LSH {
    data: HashMap<String, Vec<f32>>,
    buckets: HashMap<Hash, Vec<String>>,
    plane_norms: Vec<PlaneNorm>,
}

impl Algorithm for LSH {
    fn search(&self, query: &str, k: usize) -> Option<Vec<String>> {
        self.data.get(query).map(|v| {
            let hash = Self::hash(&self.plane_norms, v);

            let mut heap: BinaryHeap<Reverse<(usize, &Hash)>> = BinaryHeap::new();

            self.buckets
                .keys()
                .for_each(|k| heap.push(Reverse((Self::hamming_distance(&hash, k), k))));

            let mut result: Vec<String> = Vec::new();
            while let Some(Reverse((_, hash))) = heap.pop() {
                result.extend(self.buckets.get(hash).unwrap().clone().into_iter());
                if result.len() >= k {
                    break;
                }
            }

            result.truncate(k);
            result
        })
    }
}

impl LSH {
    pub fn load(data: &HashMap<String, Vec<f32>>) -> Self {
        let dimensionality = data.iter().next().unwrap().1.len();
        let num_hyperplanes = 16; // max possible buckets = 2^num_hyperplanes

        // norms of random hyperplanes
        let plane_norms: Vec<Vec<f32>> = (0..num_hyperplanes)
            .map(|_| Self::generate_plane_norm(dimensionality))
            .collect();

        let mut buckets: HashMap<Hash, Vec<String>> = HashMap::new();

        for (key, vec) in data {
            let hash = Self::hash(&plane_norms, vec);
            if let Some(entry) = buckets.get_mut(&hash) {
                entry.push(key.to_owned());
            } else {
                buckets.insert(hash, vec![key.to_owned()]);
            }
        }

        Self {
            data: data.clone(),
            buckets,
            plane_norms,
        }
    }
}

impl LSH {
    fn hash(hashers: &[PlaneNorm], v: &[f32]) -> Hash {
        hashers
            .iter()
            .map(|norm| if dot_product(v, norm) >= 0.0 { 1 } else { 0 })
            .collect()
    }

    fn generate_plane_norm(dimensionality: usize) -> PlaneNorm {
        let mut rng = rand::thread_rng();
        (0..dimensionality)
            .map(|_| rng.gen_range(-0.5..=0.5))
            .collect()
    }

    fn hamming_distance(v1: &[u8], v2: &[u8]) -> usize {
        assert_eq!(v1.len(), v2.len());
        v1.iter().zip(v2.iter()).filter(|(a, b)| a != b).count()
    }
}

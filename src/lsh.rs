use std::collections::HashMap;

use rand::Rng;

use crate::{dot_product, Algorithm};

type Hash = Vec<u8>;

pub struct LSH {
    data: HashMap<String, Vec<f32>>,
    buckets: HashMap<Hash, Vec<String>>,
    plane_norms: Vec<Vec<f32>>,
}

impl Algorithm for LSH {
    fn search(&self, query: &str, k: usize) -> Option<Vec<String>> {
        self.data.get(query).map(|v| {
            let hash = Self::to_bin(&self.plane_norms, v);

            let mut buckets_keys: Vec<&Hash> = self.buckets.keys().collect();
            buckets_keys.sort_by(|a, b| {
                Self::hamming_distance(&hash, a).cmp(&Self::hamming_distance(&hash, b))
            });

            let mut v: Vec<String> = Vec::new();
            for bucket_key in buckets_keys {
                v.extend(self.buckets.get(bucket_key).unwrap().clone().into_iter());
                if v.len() >= k {
                    break;
                };
            }

            v.truncate(k);

            v
        })
    }
}

impl LSH {
    pub fn load(data: &HashMap<String, Vec<f32>>) -> Self {
        let dimensionality = data.iter().next().unwrap().1.len();
        let mut rng = rand::thread_rng();
        let nbits = 32;

        // norms of random hyperplanes
        let plane_norms: Vec<Vec<f32>> = (0..nbits)
            .map(|_| {
                (0..dimensionality)
                    .map(|_| rng.gen_range(-0.5..=0.5))
                    .collect()
            })
            .collect();

        let mut buckets: HashMap<Hash, Vec<String>> = HashMap::new();

        for (key, vec) in data {
            let hash = Self::to_bin(&plane_norms, vec);
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
    fn to_bin(plane_norms: &[Vec<f32>], v: &[f32]) -> Hash {
        plane_norms
            .iter()
            .map(|norm| if dot_product(v, norm) > 0.0 { 1 } else { 0 })
            .collect()
    }

    fn hamming_distance(v1: &[u8], v2: &[u8]) -> usize {
        assert!(v1.len() == v2.len());
        v1.iter().zip(v2.iter()).filter(|(a, b)| a != b).count()
    }
}

use std::{
    cmp::max,
    collections::{BinaryHeap, HashMap, HashSet},
};

use rand::seq::IteratorRandom;

use crate::{distance, Algorithm, LimitedHeap, OrdItem, VectorID};

pub struct NSW {
    graph: HashMap<VectorID, Vec<VectorID>>,
    map: HashMap<VectorID, Vec<f32>>,
    dimensionality: usize,
    /// no. of search attempts
    /// failure probability decreases exponentially as m increases
    m: usize,
}

impl Algorithm for NSW {
    fn search(&self, query: &[f32], k: usize) -> Vec<VectorID> {
        self.multi_search(query, self.m, k)
            .into_iter()
            .map(|OrdItem(_, id)| id)
            .collect()
    }
}

impl NSW {
    pub fn load(data: &[(VectorID, Vec<f32>)]) -> Self {
        let dimensionality = data.first().unwrap().1.len();
        let mut s = NSW {
            graph: HashMap::with_capacity(data.len()),
            map: HashMap::with_capacity(data.len()),
            dimensionality,
            m: 5,
        };

        for item in data.iter() {
            let k = 10;
            let a = 1;
            let w = a * max(1, max(1, s.graph.len()).ilog10());
            s.insert(item, k, w as usize);
        }

        s
    }

    pub fn insert(&mut self, object: &(VectorID, Vec<f32>), k: usize, w: usize) {
        assert_eq!(self.dimensionality, object.1.len());

        self.map.insert(object.0, object.1.clone());

        if self.graph.is_empty() {
            self.graph.insert(object.0, vec![]);
            return;
        }

        if self.graph.len() <= k {
            let vertices: Vec<VectorID> = self.graph.keys().cloned().collect();
            vertices.into_iter().for_each(|v| {
                self.graph.entry(v).and_modify(|e| e.push(object.0));
                self.graph.entry(object.0).or_default().push(v);
            });
            return;
        }

        let u = self.multi_search(&object.1, w, k);

        u.iter().for_each(|&OrdItem(_, v)| {
            self.graph.entry(v).and_modify(|e| e.push(object.0));
            self.graph.entry(object.0).or_default().push(v);
        });
    }

    fn multi_search(&self, query: &[f32], m: usize, k: usize) -> Vec<OrdItem<VectorID>> {
        let mut results: LimitedHeap<OrdItem<VectorID>> = LimitedHeap::new(k);
        let mut visited = HashSet::new();
        let mut rng = rand::thread_rng();

        for _ in 0..m {
            let entry_point = self.graph.keys().choose(&mut rng).unwrap();
            if visited.contains(entry_point) {
                continue;
            }

            self.greedy_search(query, *entry_point, k, &mut results, &mut visited);

            if visited.len() == self.graph.len() {
                break;
            }
        }

        results.consume().into_sorted_vec()
    }

    fn greedy_search(
        &self,
        query: &[f32],
        entry_point: VectorID,
        k: usize,
        results: &mut LimitedHeap<OrdItem<VectorID>>,
        visited: &mut HashSet<VectorID>,
    ) {
        let mut candidates = BinaryHeap::new();

        let metric = self.metric(query, &entry_point);
        candidates.push(OrdItem(-metric, entry_point));
        results.push(OrdItem(metric, entry_point));

        while let Some(best_candidate) = candidates.pop() {
            let OrdItem(metric_cn, v_curr) = best_candidate;
            let metric_cn = -1.0 * metric_cn;

            let kth_best = if results.len() >= k {
                results.peek().unwrap().0
            } else {
                f32::INFINITY
            };

            if metric_cn > kth_best {
                break;
            }

            for v_friend in self.get_friends(&v_curr) {
                if visited.contains(v_friend) {
                    continue;
                }
                visited.insert(*v_friend);
                let metric_fr = self.metric(query, v_friend);

                candidates.push(OrdItem(-metric_fr, *v_friend));
                results.push(OrdItem(metric_fr, *v_friend));
            }

            if visited.len() == self.graph.len() {
                break;
            }
        }
    }

    fn metric(&self, query: &[f32], vertex: &VectorID) -> f32 {
        distance(query, self.map.get(vertex).unwrap())
    }

    fn get_friends(&self, vertex: &VectorID) -> &[VectorID] {
        self.graph.get(vertex).unwrap()
    }
}

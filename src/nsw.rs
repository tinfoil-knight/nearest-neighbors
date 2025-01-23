use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use rand::seq::IteratorRandom;

use crate::{distance, Algorithm, HeapItem, LimitedHeap, VectorID};

pub struct NSW {
    graph: HashMap<VectorID, Vec<VectorID>>,
    map: HashMap<VectorID, Vec<f32>>,
    dimensionality: usize,
    /// no. of search attempts
    /// failure probability decreases exponentially as m increases
    m: usize,
}

impl Algorithm for NSW {
    fn search(&self, query: &[f32], _k: usize) -> Vec<VectorID> {
        let k = 1;
        let mut heap: LimitedHeap<HeapItem<VectorID>> = LimitedHeap::new(k);
        let results = self.multi_search(query, self.m);
        for vertex in results {
            let metric = self.metric(query, &vertex);
            heap.push(HeapItem(metric, vertex));
        }
        let mut v: Vec<&HeapItem<VectorID>> = heap.iter().collect();
        v.sort();
        v.into_iter().map(|&HeapItem(_, id)| id).collect()
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
            s.insert(item.clone());
        }

        s
    }

    pub fn insert(&mut self, object: (VectorID, Vec<f32>)) {
        assert_eq!(self.dimensionality, object.1.len());

        self.map.insert(object.0, object.1.clone());

        if self.graph.is_empty() {
            self.graph.insert(object.0, vec![]);
            return;
        }

        let a = 1;
        let w = a * max(1, self.graph.len().ilog10()) as usize;
        let local_mins = self.multi_search(&object.1, w);
        let mut u: HashSet<VectorID> = HashSet::from_iter(local_mins.clone());

        for local_min in local_mins {
            u.extend(self.get_friends(&local_min).iter());
        }

        self.graph.insert(object.0, vec![]);

        let mut u_vec: Vec<&VectorID> = u.iter().collect();
        u_vec.sort_by(|a, b| {
            self.metric(&object.1, a)
                .total_cmp(&self.metric(&object.1, b))
        });

        let k = 3 * self.dimensionality;

        u_vec.iter().take(k).for_each(|&v| {
            self.graph.entry(*v).or_default().push(object.0);
            self.graph.entry(object.0).or_default().push(*v);
        });
    }

    fn multi_search(&self, query: &[f32], m: usize) -> HashSet<VectorID> {
        let mut results: HashSet<VectorID> = HashSet::new();
        let entry_points = self
            .graph
            .keys()
            .choose_multiple(&mut rand::thread_rng(), m);

        for entry_point in entry_points {
            let local_min: VectorID = self.greedy_search(query, *entry_point);
            results.insert(local_min);
        }

        results
    }

    fn greedy_search(&self, query: &[f32], entry_point: VectorID) -> VectorID {
        let v_curr = entry_point;
        let mut min_metric = self.metric(query, &v_curr);
        let mut v_next = None;

        for v_friend in self.get_friends(&v_curr) {
            let metric_fr = self.metric(query, v_friend);

            if metric_fr < min_metric {
                min_metric = metric_fr;
                v_next = Some(v_friend);
            }
        }

        v_next.map_or(v_curr, |v| self.greedy_search(query, *v))
    }

    fn metric(&self, query: &[f32], vertex: &VectorID) -> f32 {
        distance(query, self.map.get(vertex).unwrap())
    }

    fn get_friends(&self, vertex: &VectorID) -> &[VectorID] {
        self.graph.get(vertex).unwrap()
    }
}

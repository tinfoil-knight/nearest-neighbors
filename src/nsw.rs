use std::{
    cmp::max,
    collections::{BTreeSet, HashMap, HashSet},
};

use rand::seq::IteratorRandom;

use crate::{distance, Algorithm, OrdItem, VectorID};

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
        let results = self.multi_search(query, self.m);
        results.iter().take(k).map(|&OrdItem(_, id)| id).collect()
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
            s.insert(item);
        }

        s
    }

    pub fn insert(&mut self, object: &(VectorID, Vec<f32>)) {
        assert_eq!(self.dimensionality, object.1.len());

        self.map.insert(object.0, object.1.clone());

        if self.graph.is_empty() {
            self.graph.insert(object.0, vec![]);
            return;
        }

        let k = 3 * self.dimensionality;

        if self.graph.len() <= k {
            let vertices: Vec<VectorID> = self.graph.keys().cloned().collect();
            vertices.into_iter().for_each(|v| {
                self.graph.entry(v).and_modify(|e| e.push(object.0));
                self.graph.entry(object.0).or_default().push(v);
            });
            return;
        }

        let a = 1;
        let w = a * max(1, self.graph.len().ilog10()) as usize;
        let local_mins = self.multi_search(&object.1, w);
        let mut u = local_mins.clone();
        for OrdItem(_, local_min) in local_mins {
            for friend in self.get_friends(&local_min) {
                u.insert(OrdItem(self.metric(&object.1, friend), *friend));
            }
        }

        u.iter().take(k).for_each(|&OrdItem(_, v)| {
            self.graph.entry(v).and_modify(|e| e.push(object.0));
            self.graph.entry(object.0).or_default().push(v);
        });
    }

    fn multi_search(&self, query: &[f32], m: usize) -> BTreeSet<OrdItem<VectorID>> {
        let mut results = BTreeSet::new();
        let mut visited = HashSet::new();

        let entry_points = self
            .graph
            .keys()
            .choose_multiple(&mut rand::thread_rng(), m);

        for entry_point in entry_points {
            let (metric, local_min) = self.greedy_search(query, *entry_point, &mut visited);
            results.insert(OrdItem(metric, local_min));
        }

        results
    }

    fn greedy_search(
        &self,
        query: &[f32],
        entry_point: VectorID,
        visited: &mut HashSet<VectorID>,
    ) -> (f32, VectorID) {
        let mut v_curr = entry_point;
        let mut min_metric = self.metric(query, &v_curr);

        let mut v_next = v_curr;

        loop {
            for v_friend in self.get_friends(&v_curr) {
                if !visited.insert(*v_friend) {
                    continue;
                }
                let metric_fr = self.metric(query, v_friend);

                if metric_fr < min_metric {
                    min_metric = metric_fr;
                    v_next = *v_friend;
                }
            }

            if v_next == v_curr || visited.len() == self.graph.len() {
                break;
            }
            v_curr = v_next;
        }

        (min_metric, v_curr)
    }

    fn metric(&self, query: &[f32], vertex: &VectorID) -> f32 {
        distance(query, self.map.get(vertex).unwrap())
    }

    fn get_friends(&self, vertex: &VectorID) -> &[VectorID] {
        self.graph.get(vertex).unwrap()
    }
}

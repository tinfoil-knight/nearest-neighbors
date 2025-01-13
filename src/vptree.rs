use std::{collections::HashMap, f32::NAN};

use nearest_neighbors::{distance, Algorithm, BinaryTree, Node};
use rand::Rng;

pub struct VPTree {
    map: HashMap<String, Vec<f32>>,
    tree: BinaryTree<TreeItem>,
}

impl Algorithm for VPTree {
    fn search(&self, query: &str) -> Option<Vec<String>> {
        // todo:
        None
    }
}

impl VPTree {
    pub fn load(data: HashMap<String, Vec<f32>>) -> Self {
        let points: Vec<(Vec<f32>, String)> = data
            .iter()
            .map(|(word, vector)| (vector.clone(), word.clone()))
            .collect();

        let tree = Self::build(points);
        Self { map: data, tree }
    }

    fn build(mut points: Vec<(Vec<f32>, String)>) -> BinaryTree<TreeItem> {
        if points.is_empty() {
            return BinaryTree(None);
        }

        let vantage_pt = points.remove(Self::select_vp(&points));

        if points.is_empty() {
            return BinaryTree(Some(Box::new(Node {
                value: TreeItem(NAN, vantage_pt.0, vantage_pt.1),
                left: BinaryTree(None),
                right: BinaryTree(None),
            })));
        }

        let mut points_with_dist: Vec<TreeItem> = points
            .into_iter()
            .map(|pt| TreeItem(distance(&vantage_pt.0, &pt.0), pt.0, pt.1))
            .collect();

        let median = |v: &[TreeItem]| -> f32 {
            if v.is_empty() {
                return NAN;
            }
            if v.len() % 2 == 0 {
                (v[(v.len() / 2) - 1].0 + v[v.len() / 2].0) / 2.0
            } else {
                v[v.len() / 2].0
            }
        };

        points_with_dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mu = median(&points_with_dist);

        let (left, right): (Vec<TreeItem>, Vec<TreeItem>) =
            points_with_dist.into_iter().partition(|pt| pt.0 < mu);

        BinaryTree(Some(Box::new(Node {
            value: TreeItem(mu, vantage_pt.0, vantage_pt.1),
            left: Self::build(left.iter().map(|x| (x.1.clone(), x.2.clone())).collect()),
            right: Self::build(right.iter().map(|x| (x.1.clone(), x.2.clone())).collect()),
        })))
    }

    fn select_vp<T>(s: &[T]) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..s.len())
    }
}

#[derive(Debug, Clone)]
struct TreeItem(f32, Vec<f32>, String);

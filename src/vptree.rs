use core::f32;
use std::f32::NAN;

use crate::{distance, Algorithm, BinaryTree, OrdItem, LimitedHeap, Node, VectorID};
use rand::Rng;

pub struct VPTree {
    tree: BinaryTree<TreeItem>,
}

impl Algorithm for VPTree {
    fn search(&self, query: &[f32], k: usize) -> Vec<VectorID> {
        self.nearest_neighbors(query, k)
    }
}

impl VPTree {
    pub fn load(data: &[(VectorID, Vec<f32>)]) -> Self {
        let points: Vec<(Vec<f32>, VectorID)> = data
            .iter()
            .map(|(id, vector)| (vector.clone(), *id))
            .collect();
        Self {
            tree: Self::build(points),
        }
    }

    fn build(mut points: Vec<(Vec<f32>, VectorID)>) -> BinaryTree<TreeItem> {
        if points.is_empty() {
            return BinaryTree(None);
        }

        let select_vp = |size: usize| -> usize { rand::thread_rng().gen_range(0..size) };

        let vantage_pt = points.swap_remove(select_vp(points.len()));

        if points.is_empty() {
            return BinaryTree(Some(Box::new(Node {
                value: TreeItem(NAN, vantage_pt.0, vantage_pt.1),
                left: BinaryTree(None),
                right: BinaryTree(None),
            })));
        }

        let mut points_with_dist: Vec<TreeItem> = points
            .into_iter()
            .map(|(vector, id)| TreeItem(distance(&vantage_pt.0, &vector), vector, id))
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
            left: Self::build(left.iter().map(|x| (x.1.clone(), x.2)).collect()),
            right: Self::build(right.iter().map(|x| (x.1.clone(), x.2)).collect()),
        })))
    }

    fn nearest_neighbors(&self, target: &[f32], k: usize) -> Vec<VectorID> {
        let mut tau = f32::INFINITY; // threshold distance for target

        let mut stack = vec![&self.tree];
        let mut neighbors: LimitedHeap<OrdItem<(&[f32], VectorID)>> = LimitedHeap::new(k);

        while let Some(node) = stack.pop() {
            let Some(node) = &node.0 else {
                continue;
            };

            let d = distance(target, &node.value.1);

            if d < tau {
                neighbors.push(OrdItem(d, (&node.value.1, node.value.2)));
                let OrdItem(_, farthest_nearest_neighbor) = neighbors.peek().unwrap();
                tau = distance(target, farthest_nearest_neighbor.0);
            }

            if node.left.0.is_none() && node.right.0.is_none() {
                // i.e. if is leaf
                continue;
            }

            let mu = node.value.0; // division boundary for current vantage point

            // inside circle
            if d < mu {
                if d < mu + tau {
                    stack.push(&node.left);
                }
                if d >= mu - tau {
                    stack.push(&node.right);
                }
            } else {
                // the order is important because it'll reduce tau earlier
                // and prevent us from exploring unnecessary branches
                if d >= mu - tau {
                    stack.push(&node.right);
                }
                if d < mu + tau {
                    stack.push(&node.left);
                }
            };
        }

        let mut v: Vec<&OrdItem<(&[f32], VectorID)>> = neighbors.iter().collect();
        v.sort();
        v.into_iter()
            .map(|&OrdItem(_, (_vector, id))| id)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct TreeItem(f32, Vec<f32>, VectorID);

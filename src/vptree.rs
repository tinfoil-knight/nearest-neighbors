use core::f32;
use std::{cmp::Ordering, collections::HashMap, f32::NAN};

use nearest_neighbors::{distance, Algorithm, BinaryTree, LimitedHeap, Node};
use rand::Rng;

pub struct VPTree {
    map: HashMap<String, Vec<f32>>,
    tree: BinaryTree<TreeItem>,
}

impl Algorithm for VPTree {
    fn search(&self, query: &str, k: usize) -> Option<Vec<String>> {
        self.map
            .get(query)
            .map(|target| self.nearest_neighbors(target, k))
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

        let select_vp =
            |s: &[(Vec<f32>, String)]| -> usize { rand::thread_rng().gen_range(0..s.len()) };

        let vantage_pt = points.remove(select_vp(&points));

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

    fn nearest_neighbors(&self, target: &[f32], k: usize) -> Vec<String> {
        let mut tau = f32::INFINITY; // threshold distance for target

        let mut stack = vec![&self.tree];
        let mut neighbors: LimitedHeap<HeapItem> = LimitedHeap::new(k);

        while let Some(node) = stack.pop() {
            let Some(node) = &node.0 else {
                continue;
            };

            let d = distance(target, &node.value.1);

            if d < tau {
                neighbors.push(HeapItem(d, (&node.value.1, &node.value.2)));
                let HeapItem(_, farthest_nearest_neighbor) = neighbors.peek().unwrap();
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

        let mut v: Vec<&HeapItem> = neighbors.iter().collect();
        v.sort();
        v.iter().map(|HeapItem(_, x)| x.1.to_owned()).collect()
    }
}

#[derive(Debug, Clone)]
struct TreeItem(f32, Vec<f32>, String);

#[derive(Debug)]
struct HeapItem<'a>(f32, (&'a Vec<f32>, &'a String));

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

use std::cmp::Ordering;

use crate::{distance, Algorithm, BinaryTree, LimitedHeap, Node, VectorID};

pub struct KDTree {
    tree: BinaryTree<TreeItem>,
}

impl Algorithm for KDTree {
    fn search(&self, query: &[f32], k: usize) -> Vec<VectorID> {
        self.nearest_neighbors(query, k)
    }
}

impl KDTree {
    pub fn load(data: &[(VectorID, Vec<f32>)]) -> Self {
        let mut points: Vec<TreeItem> = data
            .iter()
            .map(|(id, vector)| TreeItem(vector.clone(), *id))
            .collect();

        let num_dimensions = points[0].0.len();
        Self {
            tree: Self::build(&mut points, 0, num_dimensions),
        }
    }

    fn build(points: &mut [TreeItem], depth: usize, num_dimensions: usize) -> BinaryTree<TreeItem> {
        if points.is_empty() {
            BinaryTree(None)
        } else {
            let axis = depth % num_dimensions;
            points.sort_by(|a, b| a.0[axis].partial_cmp(&b.0[axis]).unwrap());

            let median_idx = points.len() / 2;

            let (left, right) = points.split_at_mut(median_idx);
            let (value, right) = right.split_first_mut().unwrap();

            BinaryTree(Some(Box::new(Node {
                value: value.to_owned(),
                left: Self::build(left, depth + 1, num_dimensions),
                right: Self::build(right, depth + 1, num_dimensions),
            })))
        }
    }

    fn nearest_neighbors(&self, target: &[f32], k: usize) -> Vec<VectorID> {
        let num_dimensions = target.len();

        let mut k_max_heap: LimitedHeap<HeapItem> = LimitedHeap::new(k);
        let mut stack = vec![(&self.tree, 0)];

        while let Some((node, depth)) = stack.pop() {
            if let Some(point) = &node.0 {
                let axis = depth % num_dimensions;
                let dist = distance(target, &point.value.0);

                k_max_heap.push(HeapItem(dist, point.value.1));

                let next_branch;
                let opposite_branch;

                if target[axis] < point.value.0[axis] {
                    (next_branch, opposite_branch) = (&point.left, &point.right);
                } else {
                    (next_branch, opposite_branch) = (&point.right, &point.left);
                }

                stack.push((next_branch, depth + 1));

                if (k_max_heap.len() < k)
                    || (f32::powi(target[axis] - point.value.0[axis], 2)
                        < k_max_heap.peek().unwrap().0)
                {
                    stack.push((opposite_branch, depth + 1));
                }
            }
        }

        let mut v: Vec<&HeapItem> = k_max_heap.iter().collect();
        v.sort();
        v.iter().map(|&HeapItem(_, id)| *id).collect()
    }
}

#[derive(Debug, Clone)]
struct TreeItem(Vec<f32>, VectorID);

#[derive(Debug)]
struct HeapItem(f32, VectorID);

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HeapItem {}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // ignoring NaN for f32
        self.0.partial_cmp(&other.0).unwrap()
    }
}

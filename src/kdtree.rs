use std::{cmp::Ordering, collections::HashMap};

use nearest_neighbors::{distance, Algorithm, BinaryTree, LimitedHeap, Node, TOP_K_LIMIT};

pub struct KDTree {
    map: HashMap<String, Vec<f32>>,
    tree: BinaryTree<TreeItem>,
}

impl Algorithm for KDTree {
    fn search(&self, query: &str) -> Option<Vec<String>> {
        self.map
            .get(query)
            .map(|target| self.nearest_neighbors(target))
    }
}

impl KDTree {
    pub fn load(data: HashMap<String, Vec<f32>>) -> Self {
        // ? : can we avoid maintaining multiple copies of data here
        let points: Vec<TreeItem> = data
            .iter()
            .map(|(word, vector)| TreeItem(vector.clone(), word.clone()))
            .collect();

        let tree = Self::build(points, 0);
        Self { map: data, tree }
    }

    fn build(mut points: Vec<TreeItem>, depth: usize) -> BinaryTree<TreeItem> {
        if points.is_empty() {
            BinaryTree(None)
        } else {
            let num_dimensions = points[0].0.len();
            let axis = depth % num_dimensions;

            // todo: improve perf & memory usage here

            points.sort_by(|a, b| a.0[axis].partial_cmp(&b.0[axis]).unwrap());

            let median_idx = points.len() / 2;

            let (left, right) = points.split_at(median_idx);
            let (value, right) = right.split_first().unwrap();

            BinaryTree(Some(Box::new(Node {
                value: value.to_owned(),
                left: Self::build(left.to_vec(), depth + 1),
                right: Self::build(right.to_vec(), depth + 1),
            })))
        }
    }

    fn nearest_neighbors(&self, target: &[f32]) -> Vec<String> {
        let num_dimensions = target.len();

        let mut k_max_heap: LimitedHeap<HeapItem> = LimitedHeap::new(TOP_K_LIMIT);
        let mut stack = vec![(&self.tree, 0)];

        while let Some((node, depth)) = stack.pop() {
            if let Some(point) = &node.0 {
                let axis = depth % num_dimensions;
                let dist = distance(target, &point.value.0);

                k_max_heap.push(HeapItem(dist, &point.value.1));

                let next_branch;
                let opposite_branch;

                if target[axis] < point.value.0[axis] {
                    (next_branch, opposite_branch) = (&point.left, &point.right);
                } else {
                    (next_branch, opposite_branch) = (&point.right, &point.left);
                }

                stack.push((next_branch, depth + 1));

                if (k_max_heap.len() < TOP_K_LIMIT)
                    || (f32::powi(target[axis] - point.value.0[axis], 2)
                        < k_max_heap.peek().unwrap().0)
                {
                    stack.push((opposite_branch, depth + 1));
                }
            }
        }

        let mut v: Vec<&HeapItem> = k_max_heap.iter().collect();
        v.sort();
        v.iter().map(|x| x.1.to_owned()).collect()
    }
}

#[derive(Debug, Clone)]
struct TreeItem(Vec<f32>, String);

#[derive(Debug)]
struct HeapItem<'a>(f32, &'a String);

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

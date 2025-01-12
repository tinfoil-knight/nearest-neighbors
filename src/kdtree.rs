use std::collections::HashMap;

use early_ann::Algorithm;

pub struct KDTree {
    map: HashMap<String, Vec<f32>>,
    tree: BinaryTree<TreeItem>,
}

impl Algorithm for KDTree {
    fn load(data: HashMap<String, Vec<f32>>) -> Self {
        // ? : can we avoid maintaining multiple copies of data here
        let points: Vec<TreeItem> = data
            .iter()
            .map(|(word, vector)| TreeItem(vector.clone(), word.clone()))
            .collect();

        let tree = Self::build(points, 0);
        Self { map: data, tree }
    }

    fn search(&self, query: &str) -> Option<Vec<&String>> {
        if let Some(_a) = self.map.get(query) {
            todo!()
        } else {
            None
        }
    }
}

impl KDTree {
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

    pub fn len(&self) -> usize {
        Self::len_helper(&self.tree)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len_helper<T>(tree: &BinaryTree<T>) -> usize {
        if let Some(node) = &tree.0 {
            1 + Self::len_helper(&node.left) + Self::len_helper(&node.right)
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct BinaryTree<T>(Option<Box<Node<T>>>);

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug, Clone)]
struct TreeItem(Vec<f32>, String);

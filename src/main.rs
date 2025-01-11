use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead, BufReader},
};

const TOP_K_LIMIT: usize = 10;

fn main() {
    let data = load_dataset("./datasets/glove.twitter.27B.25d.txt").unwrap();
    println!("Loaded dataset. Found {} vectors.", data.len());

    let exact = Exact::load(data);
    let result = exact.search("sad");
    println!("{:?}", result);
}

fn load_dataset(path: &str) -> io::Result<HashMap<String, Vec<f32>>> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut word_map = HashMap::new();
    let mut dimensions = 0;

    for line in reader.lines() {
        let line = line?;

        let Some((word, vector_s)) = line.split_once(' ') else {
            panic!("invalid line");
        };

        let vector: Vec<f32> = vector_s
            .split_terminator(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        if dimensions == 0 {
            dimensions = vector.len();
        }

        assert_eq!(dimensions, vector.len());

        word_map.insert(word.to_string(), vector);
    }

    Ok(word_map)
}

trait Algorithm {
    fn load(data: HashMap<String, Vec<f32>>) -> Self;
    fn search(&self, query: &str) -> Option<Vec<&String>>;
}

struct Exact {
    data: HashMap<String, Vec<f32>>,
}

impl Algorithm for Exact {
    fn load(data: HashMap<String, Vec<f32>>) -> Self {
        Self { data }
    }

    fn search(&self, query: &str) -> Option<Vec<&String>> {
        if let Some(a) = self.data.get(query) {
            println!("Found word: {:?}", &a);
            let a_mag = a.iter().map(|x| x * x).sum::<f32>().sqrt();
            let mut k_min_heap: KMinHeap<HeapItem> = KMinHeap::new(TOP_K_LIMIT);

            for (key, b) in self.data.iter() {
                let b_mag = b.iter().map(|x| x * x).sum::<f32>().sqrt();
                let cosine_similarity: f32 = if a_mag == 0.0 || b_mag == 0.0 {
                    0.0
                } else {
                    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
                    dot_product / (a_mag * b_mag)
                };

                k_min_heap.insert(HeapItem(cosine_similarity, key));
                println!("key: {}, score: {}", key, cosine_similarity)
            }

            if k_min_heap.len() == 0 {
                None
            } else {
                Some(k_min_heap.get_top_k().iter().map(|v| v.1).collect())
            }
        } else {
            None
        }
    }
}

struct KMinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
    limit: usize,
}

impl<T: Ord> KMinHeap<T> {
    fn new(k: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(k),
            limit: k,
        }
    }

    fn insert(&mut self, value: T) {
        if self.heap.len() < self.limit {
            self.heap.push(Reverse(value));
        } else if let Some(Reverse(top)) = self.heap.peek() {
            if &value > top {
                self.heap.pop();
                self.heap.push(Reverse(value))
            }
        }
    }

    fn get_top_k(&self) -> Vec<&T> {
        let mut result: Vec<&T> = self.heap.iter().map(|Reverse(v)| v).collect();
        result.sort_by(|a, b| b.cmp(a));
        result
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_min_heap_insertion() {
        let mut k_min_heap = KMinHeap::new(3);

        k_min_heap.insert(10);
        k_min_heap.insert(20);
        k_min_heap.insert(5);
        k_min_heap.insert(1);
        k_min_heap.insert(15);

        let top_k = k_min_heap.get_top_k();

        assert_eq!(top_k.len(), 3);

        assert_eq!(*top_k[0], 20);
        assert_eq!(*top_k[1], 15);
        assert_eq!(*top_k[2], 10);
    }
}

#[derive(Debug, Clone)]
struct HeapItem<'a>(f32, &'a String);

impl PartialEq for HeapItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HeapItem<'_> {}

impl PartialOrd for HeapItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for HeapItem<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // ignoring NaN for f32
        self.partial_cmp(other).unwrap()
    }
}

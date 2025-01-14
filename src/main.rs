use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub mod exact;
pub mod kdtree;
pub mod vptree;

use exact::Exact;
use kdtree::KDTree;
use nearest_neighbors::Algorithm;
use vptree::VPTree;

fn main() {
    let data = load_dataset("./datasets/glove.twitter.27B.25d.txt").unwrap();
    println!("Loaded dataset. Found {} vectors.", data.len());

    let args: Vec<String> = env::args().collect();
    let algorithm = get_search_algorithm(args.get(1), data);
    let results = algorithm.search("happy", 5);
    println!("Results: {:?}", results);
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

fn get_search_algorithm(
    flag: Option<&String>,
    data: HashMap<String, Vec<f32>>,
) -> Box<dyn Algorithm> {
    let flag = flag
        .map_or("--exact", |v| v)
        .strip_prefix("--")
        .map_or("exact", |v| v);

    match flag {
        "exact" => Box::new(Exact::load(data)),
        "kdtree" => Box::new(KDTree::load(data)),
        "vptree" => Box::new(VPTree::load(data)),
        _ => Box::new(Exact::load(data)),
    }
}

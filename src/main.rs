use std::env;

use nearest_neighbors::get_search_algorithm;
use nearest_neighbors::load_dataset;

fn main() {
    let data = load_dataset("./datasets/glove.twitter.27B.25d.txt").unwrap();
    println!("Loaded dataset. Found {} vectors.", data.len());

    let args: Vec<String> = env::args().collect();
    let flag = args
        .get(1)
        .map_or("--exact", |v| v)
        .strip_prefix("--")
        .map_or("exact", |v| v);
    let algorithm = get_search_algorithm(flag, &data);
    let results = algorithm.search("happy", 5);
    println!("Results: {:?}", results);
}

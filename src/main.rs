use std::collections::HashMap;
use std::env;

use argh::FromArgs;

use nearest_neighbors::get_search_algorithm;
use nearest_neighbors::load_dataset;
use nearest_neighbors::VectorID;

#[derive(FromArgs)]
/// Configuration
struct Config {
    /// algorithm to use (optional)
    #[argh(option, short = 'a', default = "String::from(\"exact\")")]
    algorithm: String,

    /// dataset path
    #[argh(
        option,
        default = "env::var(\"DATASET_PATH\").expect(\"env DATASET_PATH should be set if path not provided\")"
    )]
    path: String,

    /// query
    #[argh(option, short = 'q')]
    query: String,
}

fn main() {
    let config: Config = argh::from_env();

    let data = load_dataset(&config.path).unwrap();
    let formatted_data: Vec<(VectorID, Vec<f32>)> = data
        .clone()
        .iter()
        .enumerate()
        .map(|(idx, (_str, vector))| (idx, vector.clone()))
        .collect();
    let hashmap: HashMap<String, Vec<f32>> = data.clone().into_iter().collect();
    println!("Loaded dataset. Found {} vectors.", formatted_data.len());

    let algorithm = get_search_algorithm(&config.algorithm, &formatted_data);
    let query_vector = hashmap
        .get(&config.query)
        .expect("query key not in dataset, can't resolve key to vector");
    let result_ids = algorithm.search(query_vector, 5);
    let results: Vec<String> = result_ids
        .iter()
        .map(|id| data.get(*id).unwrap().0.clone())
        .collect();
    println!("Results: {:?}", results);
}

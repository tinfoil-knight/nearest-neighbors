use argh::FromArgs;

use nearest_neighbors::get_search_algorithm;
use nearest_neighbors::load_dataset;

#[derive(FromArgs)]
/// Configuration
struct Config {
    /// algorithm to use (optional)
    #[argh(option, short = 'a', default = "String::from(\"exact\")")]
    algorithm: String,

    /// dataset path
    #[argh(option)]
    path: String,

    /// query
    #[argh(option, short = 'q')]
    query: String,
}

fn main() {
    let config: Config = argh::from_env();

    let data = load_dataset(&config.path).unwrap();
    println!("Loaded dataset. Found {} vectors.", data.len());

    let algorithm = get_search_algorithm(&config.algorithm, &data);
    let results = algorithm.search(&config.query, 5);
    println!("Results: {:?}", results);
}

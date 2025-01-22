use std::env;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::seq::SliceRandom;

use nearest_neighbors::{load_dataset, Algorithm};

use nearest_neighbors::{exact::Exact, kdtree::KDTree, lsh::LSH, vptree::VPTree, VectorID};

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("Method::Search");
    let path = env::var("DATASET_PATH").expect("env DATASET_PATH should be set");
    let data: Vec<(VectorID, Vec<f32>)> = load_dataset(&path)
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(idx, (_str, vector))| (idx, vector))
        .collect();
    println!("Loaded dataset. Found {} vectors.", data.len());

    let query_keys: Vec<Vec<f32>> = data
        .choose_multiple(&mut rand::thread_rng(), data.len() / 5)
        .map(|x| x.1.clone())
        .collect();

    let (exact, kdtree, vptree, lsh) = (
        Exact::load(&data),
        KDTree::load(&data),
        VPTree::load(&data),
        LSH::load(&data),
    );

    let mut rng = rand::thread_rng();

    for k in [1, 5, 10, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("Exact", k), k, |b, k| {
            b.iter(|| exact.search(query_keys.choose(&mut rng).unwrap(), *k))
        });

        group.bench_with_input(BenchmarkId::new("KDTree", k), k, |b, k| {
            b.iter(|| kdtree.search(query_keys.choose(&mut rng).unwrap(), *k))
        });

        group.bench_with_input(BenchmarkId::new("VPTree", k), k, |b, k| {
            b.iter(|| vptree.search(query_keys.choose(&mut rng).unwrap(), *k))
        });

        group.bench_with_input(BenchmarkId::new("LSH", k), k, |b, k| {
            b.iter(|| lsh.search(query_keys.choose(&mut rng).unwrap(), *k))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_search);
criterion_main!(benches);

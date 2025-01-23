use std::env;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use nearest_neighbors::exact::Exact;
use nearest_neighbors::kdtree::KDTree;
use nearest_neighbors::load_dataset;

use nearest_neighbors::lsh::LSH;
use nearest_neighbors::vptree::VPTree;
use nearest_neighbors::{nsw::NSW, VectorID};

fn bench_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("Method::Load");
    group.sample_size(10);

    let path = env::var("DATASET_PATH").expect("env DATASET_PATH should be set");
    let mut data: Vec<(VectorID, Vec<f32>)> = load_dataset(&path)
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(idx, (_str, vector))| (idx, vector))
        .collect();

    data.truncate(50_000);

    let l = data.len();

    group.bench_function(BenchmarkId::new("Exact", l), |b| {
        b.iter(|| Exact::load(&data))
    });

    group.bench_function(BenchmarkId::new("KDTree", l), |b| {
        b.iter(|| KDTree::load(&data))
    });

    group.bench_function(BenchmarkId::new("VPTree", l), |b| {
        b.iter(|| VPTree::load(&data))
    });

    group.bench_function(BenchmarkId::new("LSH", l), |b| b.iter(|| LSH::load(&data)));

    group.bench_function(BenchmarkId::new("NSW", l), |b| b.iter(|| NSW::load(&data)));

    group.finish();
}

criterion_group!(benches, bench_load);
criterion_main!(benches);

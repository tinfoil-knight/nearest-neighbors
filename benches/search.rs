use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::seq::IteratorRandom;

use nearest_neighbors::{load_dataset, Algorithm};

use nearest_neighbors::{exact::Exact, kdtree::KDTree, lsh::LSH, vptree::VPTree};

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("Method::Search");

    let data = load_dataset("./datasets/glove.twitter.27B.25d.txt").unwrap();
    println!("Loaded dataset. Found {} vectors.", data.len());

    let query_keys = data
        .keys()
        .choose_multiple(&mut rand::thread_rng(), data.len() / 5);

    let (exact, kdtree, vptree, lsh) = (
        Exact::load(&data),
        KDTree::load(&data),
        VPTree::load(&data),
        LSH::load(&data),
    );

    let mut rng = rand::thread_rng();

    for k in [1, 5, 10, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("Exact", k), k, |b, k| {
            b.iter(|| exact.search(query_keys.iter().choose(&mut rng).unwrap(), *k))
        });

        group.bench_with_input(BenchmarkId::new("KDTree", k), k, |b, k| {
            b.iter(|| kdtree.search(query_keys.iter().choose(&mut rng).unwrap(), *k))
        });

        group.bench_with_input(BenchmarkId::new("VPTree", k), k, |b, k| {
            b.iter(|| vptree.search(query_keys.iter().choose(&mut rng).unwrap(), *k))
        });

        group.bench_with_input(BenchmarkId::new("LSH", k), k, |b, k| {
            b.iter(|| lsh.search(query_keys.iter().choose(&mut rng).unwrap(), *k))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_search);
criterion_main!(benches);

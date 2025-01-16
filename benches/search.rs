use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::IteratorRandom;

use nearest_neighbors::{load_dataset, Algorithm};

use nearest_neighbors::exact::Exact;
use nearest_neighbors::kdtree::KDTree;
use nearest_neighbors::vptree::VPTree;

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("Method::Search");

    let data = load_dataset("./datasets/glove.twitter.27B.25d.txt").unwrap();
    println!("Loaded dataset. Found {} vectors.", data.len());

    let query_keys = data
        .keys()
        .choose_multiple(&mut rand::thread_rng(), data.len() / 5);

    let k = 5; // no. of neighbours

    let (exact, kdtree, vptree) = (Exact::load(&data), KDTree::load(&data), VPTree::load(&data));

    let mut rng = rand::thread_rng();

    group.bench_function("Exact", |b| {
        b.iter(|| {
            let key = query_keys.iter().choose(&mut rng).unwrap();
            exact.search(key, k)
        })
    });

    group.bench_function("KDTree", |b| {
        b.iter(|| {
            let key = query_keys.iter().choose(&mut rng).unwrap();
            kdtree.search(key, k)
        })
    });

    group.bench_function("VPTree", |b| {
        b.iter(|| {
            let key = query_keys.iter().choose(&mut rng).unwrap();
            vptree.search(key, k)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_search);
criterion_main!(benches);

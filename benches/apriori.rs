use apriori::apriori;
use criterion::{criterion_group, criterion_main, Criterion};

// function to benchmark the performance of our apriori algorithm
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("apriori", |b| b.iter(|| apriori(vec![vec![1, 2, 3, 4],
    vec![1, 2, 4],
    vec![1, 2],
    vec![2, 3, 4],
    vec![2, 3],
    vec![3, 4],
    vec![2, 4]], 0.4)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); // display the results
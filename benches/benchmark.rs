use criterion::*;

fn bench(c: &mut Criterion) {
    c.bench_function("bench", |b| b.iter(|| {
    }));
}

criterion_group!(
    benches,
    bench,
);
criterion_main!(benches);
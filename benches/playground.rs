use criterion::*;

fn bench(c: &mut Criterion) {
    c.bench_function("playground", |b| b.iter(|| {
    }));
}

criterion_group!(
    benches,
    bench,
);
criterion_main!(benches);
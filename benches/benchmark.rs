use criterion::*;

fn test(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(|| {
    }));
}

criterion_group!(
    benches,
    test,
);
criterion_main!(benches);
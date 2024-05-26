use fibra::*;
use criterion::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn run(c: &mut Criterion) {
    let app = Arc::new(fibra!{});
    let con = Arc::new(Connection::new());

    c.bench_function("run_baseline", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::new();
            let ctx = Context::new(app.clone(), con.clone(), req);

            let _ = black_box(ctx.next().await);
        });
    });
}

fn req(_c: &mut Criterion) {

}

fn res(_c: &mut Criterion) {

}

fn ctx(_c: &mut Criterion) {

}

fn app(_c: &mut Criterion) {

}

criterion_group!(
    benches,
    run,
    req,
    res,
    ctx,
    app,
);
criterion_main!(benches);
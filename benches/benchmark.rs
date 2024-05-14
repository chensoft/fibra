use fibra::*;
use criterion::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn run(c: &mut Criterion) {
    let app = Arc::new(fibra!{
        get("/api/v1/user") => "user1",
        get("/api/v2/user") => "user2",
        post("/api/v3/user") => "user3",
    });
    let con = Arc::new(Connection::default());

    c.bench_function("run_baseline", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::default().uri("http://example.com/api/v2/user");
            let ctx = Context::from((app.clone(), con.clone(), req));

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
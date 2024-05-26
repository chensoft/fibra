use fibra::*;
use criterion::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn run(c: &mut Criterion) {
    let con = Arc::new(Connection::new());
    let app_0 = Arc::new(fibra!{});
    let app_1 = Arc::new(fibra!{
        get("/") => "hello",
    });
    let app_8 = Arc::new(fibra!{
        get("/login") => "login",
        get("/logout") => "logout",
        get("/profile") => "profile",
        get("/profile/edit") => "profile/edit",
        get("/cart") => "cart",
        get("/cart/checkout") => "cart/checkout",
        get("/products/1/reviews") => "products/1/reviews",
        get("/products/1/reviews/new") => "products/1/reviews/new",
    });
    let app_16 = Arc::new(fibra!{
        get("/login") => "login",
        get("/logout") => "logout",
        get("/profile") => "profile",
        get("/profile/edit") => "profile/edit",
        get("/cart") => "cart",
        get("/cart/checkout") => "cart/checkout",
        get("/products/1/reviews") => "products/1/reviews",
        get("/products/1/reviews/new") => "products/1/reviews/new",
        get("/register") => "register",
        get("/password/reset") => "password/reset",
        get("/dashboard") => "dashboard",
        get("/settings") => "settings",
        get("/orders") => "orders",
        get("/orders/1") => "orders/1",
        get("/products/1/questions") => "products/1/questions",
        get("/products/1/questions/new") => "products/1/questions/new",
    });

    c.bench_function("run_baseline", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::new();
            let ctx = Context::new(app_0.clone(), con.clone(), req);
            let _ = black_box(ctx.next().await);
        });
    });

    c.bench_function("run_routes_1", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::new().uri(Uri::from_static("http://localhost:3000"));
            let ctx = Context::new(app_1.clone(), con.clone(), req);
            let _ = black_box(ctx.next().await);
        });
    });

    c.bench_function("run_routes_8_front", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::new().uri(Uri::from_static("http://localhost:3000/login"));
            let ctx = Context::new(app_8.clone(), con.clone(), req);
            let _ = black_box(ctx.next().await);
        });
    });

    c.bench_function("run_routes_8_middle", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::new().uri(Uri::from_static("http://localhost:3000/cart"));
            let ctx = Context::new(app_8.clone(), con.clone(), req);
            let _ = black_box(ctx.next().await);
        });
    });

    c.bench_function("run_routes_8_back", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::new().uri(Uri::from_static("http://localhost:3000/products/1/reviews/new"));
            let ctx = Context::new(app_8.clone(), con.clone(), req);
            let _ = black_box(ctx.next().await);
        });
    });

    c.bench_function("run_routes_16", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            let req = Request::new().uri(Uri::from_static("http://localhost:3000/products/1/questions/new"));
            let ctx = Context::new(app_16.clone(), con.clone(), req);
            let _ = black_box(ctx.next().await);
        });
    });
}

fn req(c: &mut Criterion) {
    c.bench_function("req_empty", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(Request::new());
        });
    });

    c.bench_function("req_hyper", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(hyper::Request::builder().uri("http://localhost:3000/products/1/questions/new").body(Body::default()).unwrap());
        });
    });

    c.bench_function("req_build", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(Request::from(hyper::Request::builder().uri("http://localhost:3000/products/1/questions/new").body(Body::default()).unwrap()));
        });
    });
}

fn res(c: &mut Criterion) {
    c.bench_function("res_empty", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(Response::new());
        });
    });

    c.bench_function("res_full", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(Response::from((Status::OK, mime::TEXT_HTML_UTF_8, "<html></html>")));
        });
    });

    c.bench_function("res_status_body", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(Response::from((Status::OK, "<html></html>")));
        });
    });

    c.bench_function("res_status", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(Response::from(Status::OK));
        });
    });

    c.bench_function("res_body", |b| {
        b.to_async(&Runtime::new().unwrap()).iter(|| async {
            black_box(Response::from("<html></html>"));
        });
    });
}

criterion_group!(
    benches,
    run,
    req,
    res,
);
criterion_main!(benches);
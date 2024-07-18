use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();

    app.mount(addon::ReqID::new());
    app.mount(addon::Logger::new());

    // the request handler
    // $ http -v localhost:3000/about
    app.get("/about", about)?;

    // todo rewrite using middleware

    // rewrite request manually
    // $ http -v localhost:3000/about.html
    // $ http -v localhost:3000/about.html?name=Alice
    app.get("/about.html", |ctx: Context| {
        let query = match ctx.req().query().is_empty() {
            true => "".to_string(),
            false => format!("?{}", ctx.req().query()),
        };
        let href = format!("{}://{}:{}/about{}", ctx.scheme(), ctx.host(), ctx.port(), query);
        async { ctx.rewrite(href, None).await }
    })?;

    app.bind(3000)?;
    app.run().await
}

async fn about(ctx: Context) -> FibraResult<Response> {
    let name = match ctx.query("name") {
        "" => "Visitor",
        val => val,
    };

    Ok(format!("Welcome, {}!", name).into())
}
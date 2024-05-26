use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();

    app.mount(addon::ReqID::new());
    app.mount(addon::Logger::new());

    // plain text
    // $ http -v localip.cc:3000
    app.get("/", "Hello World!")?;

    // named param
    // $ http -v localip.cc:3000/user/abcde
    app.get("/user/:id", |ctx: Context| {
        let id = ctx.param("id").to_string();
        async { Ok(id.into()) }
    })?;

    // glob matching
    // $ http -v localip.cc:3000/about.html
    app.get("/*.html", |ctx: Context| {
        let name = ctx.param("*").to_string();
        async { Ok((mime::TEXT_HTML_UTF_8, name).into()) }
    })?;

    // regex matching
    // $ http -v localip.cc:3000/id/123-45-6789
    app.get(r"/id/{id:\d\d\d-\d\d-\d\d\d\d}", |ctx: Context| {
        let id = ctx.param("id").to_string();
        async { Ok(id.into()) }
    })?;

    // listen and serve
    app.bind(3000)?;
    app.run().await
}
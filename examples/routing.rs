use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.mount(addon::Logger::default());

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
        async { Ok(name.into()) }
    })?;

    // regex matching
    // $ http -v localip.cc:3000/id/123-45-6789
    app.get(r"/id/{id:\d\d\d-\d\d-\d\d\d\d}", |ctx: Context| {
        let id = ctx.param("id").to_string();
        async { Ok(id.into()) }
    })?;

    // listen and serve
    app.bind("0.0.0.0:3000")?;
    app.run().await
}
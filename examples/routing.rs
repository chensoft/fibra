use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();

    app.mount(addon::ReqID::new());
    app.mount(addon::Logger::new());

    // plain text
    // $ http -v localhost:3000
    app.get("/", "Hello World!")?;

    // named param
    // $ http -v localhost:3000/user/abcde
    app.get("/user/:id", |ctx: Context| {
        let id = ctx.param("id").to_string();
        async { id }
    })?;

    // glob matching
    // $ http -v localhost:3000/about.html
    app.get("/*.html", |ctx: Context| {
        let name = ctx.param("*").to_string();
        async { (mime::TEXT_HTML_UTF_8, name) }
    })?;

    // regex matching
    // $ http -v localhost:3000/id/123-45-6789
    app.get(r"/id/{id:\d\d\d-\d\d-\d\d\d\d}", |ctx: Context| {
        let id = ctx.param("id").to_string();
        async { id }
    })?;

    // listen and serve
    app.bind(3000)?;
    app.run().await
}
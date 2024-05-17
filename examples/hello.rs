use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    // create a main router with a predefined logger
    let mut app = Fibra::new();
    // app.mount(addon::Logger::default());
    app.route("/", "It Works!")?;
    // app.route("/index.html", |ctx: Context| async { ctx.rewrite("/", "todo").await })?;

    // create a subrouter with a subdomain that starts with 'api'
    let api = app.group("/")?;
    // api.limit().host("api.*");
    api.route("/", "api")?;

    // create api/v1's subrouter
    let v1 = api.group("/v1")?;
    v1.route("/", "v1")?;
    v1.route("/user", v1_user)?;

    // create api/v2's subrouter
    let v2 = api.group("/v2")?;
    v2.route("/", "v2")?;
    v2.route("/user", v2_user)?;

    // bind on a port and run the server
    app.bind("0.0.0.0:3000")?;
    app.run().await
}

async fn v1_user(_ctx: Context) -> FibraResult<Response> {
    Ok("".into())
}

async fn v2_user(_ctx: Context) -> FibraResult<Response> {
    Ok("".into())
}
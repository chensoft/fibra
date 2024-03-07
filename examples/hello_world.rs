use veloce::*;

#[tokio::main]
async fn main() -> Result<()> {
    // create a main router with a logger
    let mut app = Veloce::default();
    app.mount(addons::Logger::default());
    // app.route("/", get!(app_root)); // todo change to app.route("/").get(app_root) no macros here
    // todo app.route("/index.html").get(|ctx| ctx.rewrite("/"));

    // create a subrouter with a subdomain that starts with 'api'
    let api = app.group("/");
    api.limit().domain("api.*");
    // api.route("/", get!(api_root));

    let v1 = api.group("/v1");
    // v1.route("/", get!(v1_root));
    // v1.route("/user", all!(v1_user));

    let v2 = api.group("/v2");
    // v2.route("/", get!(v2_root));
    // v2.route("/user", all!(v2_user));

    app.bind("0.0.0.0:3000")?;
    app.bind("0.0.0.0:3333")?;
    app.run().await
}

async fn app_root(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from("It Works!");
    Ok(())
}

async fn api_root(_ctx: &mut Context) -> Result<()> {
    Err(StatusCode::NO_CONTENT.into_error())
}

async fn v1_root(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}

async fn v1_user(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}

async fn v2_root(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}

async fn v2_user(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}
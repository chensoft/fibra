use veloce::*;

#[tokio::main]
async fn main() -> Result<()> {
    // create an app with a logger
    let mut app = Veloce::default();
    app.mount(addons::Logger::default());
    app.route("/", get!(app_root));

    // todo create api subdomain router
    let api = app.group("/").domains(vec!["api.*"]);
    api.route("/", get!(api_root));

    let v1 = api.group("/v1");
    v1.route("/", all!(api_v1_root));
    v1.route("/user", all!(api_v1_user));

    let mut v2 = Veloce::default();
    v2.route("/", get!(api_v2_root));
    v2.route("/user", all!(api_v2_user));
    api.route("/v2", v2);

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

async fn api_v1_root(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}

async fn api_v1_user(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}

async fn api_v2_root(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}

async fn api_v2_user(ctx: &mut Context) -> Result<()> {
    *ctx.res.body_mut() = Body::from(ctx.req.uri().to_string());
    Ok(())
}
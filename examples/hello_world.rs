use veloce::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = Veloce::default();
    api.mount(addons::Logger::from_millis());
    api.route("/", get!(api_root));

    // todo /api use subdomain filter
    api.group("/api/v1", |v1| {
        v1.route("/", all!(api_v1_root));
        v1.route("/user", all!(api_v1_user));
    });

    let mut v2 = Veloce::default();
    v2.route("/", all!(api_v2_root));
    v2.route("/user", all!(api_v2_user));
    api.route("/api/v2", v2);

    api.bind("0.0.0.0:3000").await?;
    api.bind("0.0.0.0:3333").await?;
    api.run().await
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
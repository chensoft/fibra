use veloce::addons;
use veloce::filter::*;
use veloce::{Veloce, Result, Context};

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = Veloce::default();
    api.mount(addons::Logger::from_millis());
    api.route("/", api_root);

    // todo /api use subdomain filter
    api.group("/api/v1", None, |v1| {
        v1.route("/", api_v1_root);
        v1.route("/user", api_v1_user);
    });

    let mut v2 = Veloce::default();
    v2.route("/", api_v2_root);
    v2.route("/user", api_v2_user);
    api.route("/api/v2", v2);

    api.bind("0.0.0.0:3000").await?;
    api.bind("0.0.0.0:3333").await?;
    api.run().await
}

async fn api_root(mut ctx: Context) -> Result<Context> {
    if !ctx.is_get() {
        return ctx.next().await;
    }

    *ctx.res.body_mut() = http::Body::from(ctx.req.uri().to_string());

    Ok(ctx)
}

async fn api_v1_root(mut ctx: Context) -> Result<Context> {
    *ctx.res.body_mut() = http::Body::from(ctx.req.uri().to_string());

    Ok(ctx)
}

async fn api_v1_user(mut ctx: Context) -> Result<Context> {
    *ctx.res.body_mut() = http::Body::from(ctx.req.uri().to_string());

    Ok(ctx)
}

async fn api_v2_root(mut ctx: Context) -> Result<Context> {
    *ctx.res.body_mut() = http::Body::from(ctx.req.uri().to_string());

    Ok(ctx)
}

async fn api_v2_user(mut ctx: Context) -> Result<Context> {
    *ctx.res.body_mut() = http::Body::from(ctx.req.uri().to_string());

    Ok(ctx)
}
use veloce::plugin;
use veloce::filter::*;
use veloce::{Veloce, Result, Context};

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = Veloce::default();
    api.mount(plugin::Logger);
    api.route("/", api_root);
    api.rewrite("/rewrite", http::Uri::from_static("/"));
    api.redirect("/redirect", http::Uri::from_static("/"), None);

    // // todo /api use subdomain filter
    // let v1 = api.group("/api/v1", Config::default());
    // v1.route("/", api_v1_root);
    // v1.route("/user", api_v1_user);
    // 
    // let mut v2 = Veloce::default();
    // v2.route("/", api_v2_root);
    // v2.route("/user", api_v2_user);
    // api.route("/api/v2", v2);

    api.bind("0.0.0.0:3000").await?;
    api.bind("0.0.0.0:3333").await?;
    api.run().await
}

async fn api_root(ctx: Context) -> Result<Context> {
    if !ctx.is_get() {
        return ctx.next().await;
    }

    Ok(ctx)
}

// async fn api_v1_root(mut ctx: Context) -> Result<Context> {
//     *ctx.res.status_mut() = http::StatusCode::NOT_FOUND;
//     Ok(ctx)
// }
// 
// async fn api_v1_user(mut ctx: Context) -> Result<Context> {
//     *ctx.res.status_mut() = http::StatusCode::NOT_FOUND;
//     Ok(ctx)
// }
// 
// async fn api_v2_root(mut ctx: Context) -> Result<Context> {
//     *ctx.res.status_mut() = http::StatusCode::NOT_FOUND;
//     Ok(ctx)
// }
// 
// async fn api_v2_user(mut ctx: Context) -> Result<Context> {
//     *ctx.res.status_mut() = http::StatusCode::NOT_FOUND;
//     Ok(ctx)
// }
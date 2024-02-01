use veloce::{Veloce, Result, Context};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = Veloce::new(None);
    // app.route("/", veloce::exts::func::Func {func: Box::new(hello)});
    app.bind(":3000").await?;
    app.run().await
}

async fn hello(mut ctx: Context) -> Result<()> {
    *ctx.res.status_mut() = http::StatusCode::NOT_FOUND;
    Ok(())
}
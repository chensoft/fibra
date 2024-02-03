use veloce::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    veloce::serve!("0.0.0.0:3000", "0.0.0.0:3030"; "/" => hello)
}

async fn hello(mut ctx: Context) -> Result<()> {
    *ctx.res.status_mut() = http::StatusCode::NOT_FOUND;
    Ok(())
}
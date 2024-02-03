use veloce::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    veloce::serve!("127.0.0.1:3000", "127.0.0.1:3030"; "/" => hello)
}

async fn hello(mut ctx: Context) -> Result<()> {
    *ctx.res.status_mut() = http::StatusCode::NOT_FOUND;
    Ok(())
}
pub mod core;
pub mod exts;

pub use http;
pub use mime;
pub use core::*;

pub async fn serve(root: Router, addr: impl tokio::net::ToSocketAddrs) -> anyhow::Result<()> {
    let mut app = Veloce::new(None);
    app.bind(addr).await?;
    app.run(root).await
}
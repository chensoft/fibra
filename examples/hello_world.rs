use veloce::{Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = Router::new();

    veloce::serve(root, ":3000").await
}
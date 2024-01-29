use veloce::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = Router::new();
    
    let mut app = Veloce::new(None);
    app.bind(":3000").await?;
    app.run(root).await
}
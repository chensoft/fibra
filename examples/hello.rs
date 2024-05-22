use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.get("/", "Hello World!")?;
    app.bind(3000)?;
    app.run().await
}
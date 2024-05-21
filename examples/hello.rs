use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.mount(addon::Logger::default());
    app.get("/", "Hello World!")?;
    app.bind("0.0.0.0:3000")?;
    app.run().await
}
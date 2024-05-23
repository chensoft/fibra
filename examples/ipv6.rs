use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.mount(addon::Logger::default());

    // ✅: $ http -v http://[::1]:3000
    // ❌: $ http -v http://127.0.0.1:3000
    app.get("/", "Hello World!")?;

    app.bind("[::1]:3000")?; // ipv6-only
    app.run().await
}
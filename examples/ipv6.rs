use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.mount(addon::Logger::default());

    // ✅: $ curl -v -6 http://localip.cc:3000
    // ❌: $ curl -v -4 http://localip.cc:3000
    app.get("/", "Hello World!")?;

    app.bind("[::1]:3000")?; // ipv6-only
    app.run().await
}
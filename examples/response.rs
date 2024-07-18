use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();

    // $ http -v localhost:3000/function
    app.get("/function", function)?;

    // $ http -v localhost:3000/closure
    app.get("/closure", |_| async { Ok("Closure".into()) })?;

    // $ http -v localhost:3000/tuple-smt
    app.get("/tuple-smt", (Status::OK, mime::APPLICATION_JSON, "{\"type\":\"Tuple SMT\"}"))?;

    // $ http -v localhost:3000/tuple-st
    app.get("/tuple-st", (Status::OK, "{\"type\":\"Tuple ST\"}"))?;

    // $ http -v localhost:3000/tuple-mt
    app.get("/tuple-mt", (mime::APPLICATION_JSON, "{\"type\":\"Tuple MT\"}"))?;

    // $ http -v localhost:3000/empty
    app.get("/empty", ())?;

    // $ http -v localhost:3000/status
    app.get("/status", Status::NO_CONTENT)?;

    // $ http -v localhost:3000/text
    app.get("/text", "Text")?;

    app.bind(3000)?;
    app.run().await
}

async fn function(_ctx: Context) -> FibraResult<Response> {
    Ok("Function".into())
}
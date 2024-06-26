use fibra::*;
use serde::Serialize;
use indexmap::IndexMap;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();

    app.mount(addon::ReqID::new());
    app.mount(addon::Logger::new());

    // <- http -v localip.cc:3000 name=echo
    // -> {"name":"echo"}
    app.all("/*", echo)?;

    app.bind(3000)?;
    app.run().await
}

#[derive(Default, Serialize)]
struct Echo {
    meta: IndexMap<&'static str, String>,
    method: String,
    scheme: String,
    version: String,
    href: String,
    host: String,
    port: u16,
    path: String,
    queries: IndexMap<String, String>,
    headers: IndexMap<String, String>,
    body: String,
}

async fn echo(mut ctx: Context) -> FibraResult<Response> {
    let mut data = Echo::default();

    // meta
    data.meta.insert("ip", ctx.conn().peeraddr_ref().ip().to_string());
    data.meta.insert("port", ctx.conn().peeraddr_ref().port().to_string());

    // uri
    data.method = ctx.method().to_string();
    data.scheme = ctx.scheme().to_string();
    data.version = format!("{:?}", ctx.version())[5..].to_string();
    data.href = ctx.href();
    data.host = ctx.host().to_string();
    data.port = ctx.port();
    data.path = ctx.path().to_string();

    // queries
    data.queries = ctx.queries().clone();

    // headers
    for (key, val) in ctx.headers() {
        data.headers.insert(key.to_string(), String::from_utf8_lossy(val.as_bytes()).to_string());
    }

    // body
    data.body = String::from_utf8_lossy(ctx.read_all().await.unwrap_or_default().as_ref()).to_string();

    Ok((mime::APPLICATION_JSON, serde_json::to_string(&data).unwrap_or_default()).into())
}
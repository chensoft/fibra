use fibra::*;
use bytes::BytesMut;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.mount(addon::Logger::new());

    // <- http -v localip.cc:3000 name=echo
    // -> {"name":"echo"}
    app.post("/", echo)?;

    app.bind(3000)?;
    app.run().await
}

async fn echo(mut ctx: Context) -> FibraResult<Response> {
    let kind = ctx.header(header::CONTENT_TYPE).cloned().unwrap_or(mime::APPLICATION_OCTET_STREAM.into_header_value());
    let mut body = BytesMut::new();

    // read chunk by chunk and limit body's size
    let limits = 1024; // 1KB
    let mut length = 0;

    while let Some(chunk) = ctx.read_chunk().await {
        length += chunk.len();

        if length > limits {
            return Ok(Status::PAYLOAD_TOO_LARGE.into());
        }

        body.extend(chunk);
    }

    Ok(Response::new().header(header::CONTENT_TYPE, kind).body(body.freeze()))
}
use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    // create a main router with a predefined logger
    let mut app = Fibra::default();
    app.mount(addon::Logger::default());
    app.route("/", app_root)?;
    app.route("/index.html", |ctx: Context| async { ctx.rewrite("/", b"todo".into()).await })?;

    // create a subrouter with a subdomain that starts with 'api'
    let api = app.group("/")?;
    api.limit().host("api.*");
    api.route("/", api_root)?;

    // create api/v1's subrouter
    let v1 = api.group("/v1")?;
    v1.route("/", v1_root)?;
    v1.route("/user", v1_user)?;

    // create api/v2's subrouter
    let v2 = api.group("/v2")?;
    v2.route("/", v2_root)?;
    v2.route("/user", v2_user)?;

    // bind on a port and run the server
    app.bind("0.0.0.0:3000")?;
    app.run().await
}

async fn app_root(_ctx: Context) -> FibraResult<Response<Body>> {
    Ok((StatusCode::OK, "It Works!").into_response())
}

async fn api_root(_ctx: Context) -> FibraResult<Response<Body>> {
    Ok((StatusCode::OK, "Api Root").into_response())
}

async fn v1_root(_ctx: Context) -> FibraResult<Response<Body>> {
    Ok((StatusCode::OK, "V1 Root").into_response())
}

async fn v1_user(_ctx: Context) -> FibraResult<Response<Body>> {
    Ok((StatusCode::OK, "User1").into_response())
}

async fn v2_root(_ctx: Context) -> FibraResult<Response<Body>> {
    Ok((StatusCode::OK, "V2 Root").into_response())
}

async fn v2_user(_ctx: Context) -> FibraResult<Response<Body>> {
    Ok((StatusCode::OK, "User2").into_response())
}
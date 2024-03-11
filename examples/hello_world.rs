use veloce::*;

#[tokio::main]
async fn main() -> Result<()> {
    // create a main router with a logger
    let mut app = Veloce::default();
    app.mount(addons::Logger::default());
    app.route("/", app_root);
    app.route("/index.html", |ctx: Context| async { ctx.rewrite("/").await });

    // create a subrouter with a subdomain that starts with 'api'
    let api = app.group("/");
    api.limit().host("api.*");
    api.route("/", api_root);

    // create api version 1 router
    let v1 = api.group("/v1");
    v1.route("/", v1_root);
    v1.route("/user", v1_user);

    // create api version 2 router
    let v2 = api.group("/v2");
    v2.route("/", v2_root);
    v2.route("/user", v2_user);

    // bind on two ports and run the server
    app.bind("0.0.0.0:3000")?;
    app.bind("0.0.0.0:3333")?;
    app.run().await
}

async fn app_root(_ctx: Context) -> Result<Response<Body>> {
    Ok((StatusCode::OK, "It Works!").into_response())
}

async fn api_root(_ctx: Context) -> Result<Response<Body>> {
    todo!()
}

async fn v1_root(_ctx: Context) -> Result<Response<Body>> {
    todo!()
}

async fn v1_user(_ctx: Context) -> Result<Response<Body>> {
    todo!()
}

async fn v2_root(_ctx: Context) -> Result<Response<Body>> {
    todo!()
}

async fn v2_user(_ctx: Context) -> Result<Response<Body>> {
    todo!()
}
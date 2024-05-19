use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    // create fibra app
    let mut app = Fibra::new();

    // mount some middlewares
    app.mount(addon::Logger::default());

    // use string as the response
    // cmd: http -v localip.cc:3000
    app.get("/", "index")?.limit().subdomain("");

    // use closure as the response and rewrite it to the above handler
    // cmd: http -v localip.cc:3000/index.html
    app.get("/index.html", |ctx: Context| async { ctx.rewrite("/", "").await })?;

    // create a subrouter with a subdomain 'api'
    // cmd: http -v api.localip.cc:3000
    let api = app.group("/")?;
    api.limit().subdomain("api");
    api.get("/", "api")?;

    // create a endpoint
    // cmd: http -v api.localip.cc:3000/v1
    // cmd: http -v api.localip.cc:3000/v1/user
    let v1 = api.group("/v1")?;
    v1.get("/", "v1")?;
    v1.get("/user", v1_user)?;

    // create a endpoint
    // cmd: http -v api.localip.cc:3000/v2
    // cmd: http -v api.localip.cc:3000/v2/user
    let v2 = api.group("/v2")?;
    v2.get("/", "v2")?;
    v2.get("/user", v2_user)?;

    // handle 404 NOT_FOUND and other errors
    // cmd: http -v localip.cc:3000/missing
    app.catch(|err| match err {
        FibraError::PathNotFound => (Status::NOT_FOUND, "Oops! Page not found.").into(),
        _ => Status::INTERNAL_SERVER_ERROR.into(),
    });

    // bind on a port and run the server
    app.bind("0.0.0.0:3000")?;
    app.run().await
}

async fn v1_user(_ctx: Context) -> FibraResult<Response> {
    Ok("I'm User1".into())
}

async fn v2_user(_ctx: Context) -> FibraResult<Response> {
    Ok("I'm User2".into())
}
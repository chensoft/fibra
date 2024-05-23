use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.mount(addon::Logger::new());

    // create a subrouter with a subdomain 'alice'
    let alice = app.group("/")?;
    alice.limit().subdomain("alice");

    alice.get("/", "This is Alice's website")?; // $ http -v alice.localip.cc:3000
    alice.get("/users", alice_users)?;                  // $ http -v alice.localip.cc:3000/users

    alice.catch(|err| match err {
        FibraError::PathNotFound => (Status::NOT_FOUND, "Oops! No contents found on Alice's website.").into(),
        _ => Status::INTERNAL_SERVER_ERROR.into(),
    });

    // create a subrouter with a subdomain 'bob'
    let bob = app.group("/")?;
    bob.limit().subdomain("bob");

    bob.get("/", "This is Bob's website")?; // $ http -v bob.localip.cc:3000
    bob.get("/users", bob_users)?;                  // $ http -v bob.localip.cc:3000/users

    bob.catch(|err| match err {
        FibraError::PathNotFound => (Status::NOT_FOUND, "Oops! No contents found on Bob's website.").into(),
        _ => Status::INTERNAL_SERVER_ERROR.into(),
    });

    // other requests will fall into here
    // $ http -v localip.cc:3000
    app.get("/", "Try http://alice.localip.cc:3000")?;

    // handle 404 NOT_FOUND and other errors
    // $ http -v localip.cc:3000/invalid
    app.catch(|err| match err {
        FibraError::PathNotFound => (Status::NOT_FOUND, "Oops! Page not found.").into(),
        _ => Status::INTERNAL_SERVER_ERROR.into(),
    });

    app.bind("0.0.0.0:3000")?;
    app.run().await
}

async fn alice_users(_ctx: Context) -> FibraResult<Response> {
    Ok(Response::new().json(vec![1001, 1002, 1003]))
}

async fn bob_users(_ctx: Context) -> FibraResult<Response> {
    Ok(Response::new().json(vec![2001, 2002, 2003]))
}
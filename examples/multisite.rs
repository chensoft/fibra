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

    alice.catch(|res, _err| {
        res.body("Oops! No contents found on Alice's website.")
    });

    // create a subrouter with a subdomain 'bob'
    let bob = app.group("/")?;
    bob.limit().subdomain("bob");

    bob.get("/", "This is Bob's website")?; // $ http -v bob.localip.cc:3000
    bob.get("/users", bob_users)?;                  // $ http -v bob.localip.cc:3000/users

    bob.catch(|res, _err| {
        res.body("Oops! No contents found on Bob's website.")
    });

    // other requests will fall into here
    // $ http -v localip.cc:3000
    app.get("/", "Try http://alice.localip.cc:3000")?;

    // handle 404 NOT_FOUND and other errors
    // $ http -v localip.cc:3000/invalid
    app.catch(|res, _err| {
        res.body("Oops! Page not found.")
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
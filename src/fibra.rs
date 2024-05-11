//! The Router
use crate::route::*;
use crate::types::*;

/// Fibra, the core struct in this crate, acts as the central processor, handling and directing
/// all routing logic and operations. It processes incoming requests, matching them to predefined
/// handlers. Fibra also handles middlewares and errors, making it essential for building flexible
/// and robust web apps.
///
/// # Examples
///
/// ```no_run
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut app = Fibra::new();
///     app.get("/", "Hello World!")?;
///     app.get("/user/:id", |ctx: Context| async move { Ok(ctx.param("id").to_string().into()) })?;
///     app.bind("0.0.0.0:3000")?;
///     app.run().await
/// }
/// ```
pub struct Fibra {
    /// Limiter is used to determine if certain preconditions are met. If the conditions
    /// are not satisfied, no further processing will occur within this router.
    limiter: Option<Limiter>,

    /// Catcher is used to catch all errors and panics to prevent the program from crashing,
    /// if the Subrouter does not assign this field, the Parent Router will handle it.
    catcher: Option<Catcher>,

    /// Mounted is used to store HTTP handlers. Any type that implements the Handler Trait can
    /// become a handler. Middlewares that are predefined in the **addon** folder can also be
    /// added as handlers.
    mounted: Vec<BoxHandler>,

    /// Sockets is used to store all TCP listeners. We support listening on multiple addresses
    /// simultaneously. You can achieve this by calling the **bind** method multiple times.
    sockets: Vec<Socket>,
}

impl Fibra {
    /// Create a fibra router
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a route for GET method
    pub fn get(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler)?.limit().method(Method::GET);
        Ok(self)
    }

    /// Register a route for POST method
    pub fn post(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler)?.limit().method(Method::POST);
        Ok(self)
    }

    /// Register a route for PUT method
    pub fn put(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler)?.limit().method(Method::PUT);
        Ok(self)
    }

    /// Register a route for DELETE method
    pub fn delete(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler)?.limit().method(Method::DELETE);
        Ok(self)
    }

    /// Register a route for PATCH method
    pub fn patch(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler)?.limit().method(Method::PATCH);
        Ok(self)
    }

    /// Register a route for all methods
    pub fn all(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler)?;
        Ok(self)
    }

    /// Register a route
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::Arc;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut app = Fibra::new();
    ///
    ///     app.get("/api/v1/user", "user1")?;
    ///     app.get("/api/v2/user", "user2")?;
    ///
    ///     // mock a real request and check the response body
    ///     let con = Connection::default();
    ///     let req = Request::default().uri("http://example.com/api/v2/user");
    ///     let ctx = Context::from((Arc::new(app), Arc::new(con), req));
    ///
    ///     assert_eq!(ctx.next().await?.body_all().await?, "user2");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn route(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Routine> {
        self.ensure::<Matcher>().insert(path, handler)
    }

    /// Register a subrouter
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::Arc;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut app = Fibra::new();
    ///     let api = app.group("/api")?;
    ///
    ///     let v1 = api.group("/v1")?;
    ///     v1.get("/user", "user1")?;
    ///
    ///     let v2 = api.group("/v2")?;
    ///     v2.get("/user", "user2")?;
    ///
    ///     // mock a real request
    ///     let con = Connection::default();
    ///     let req = Request::default().uri("http://example.com/api/v2/user");
    ///     let mut ctx = Context::from((Arc::new(app), Arc::new(con), req));
    ///
    ///     assert_eq!(ctx.next().await?.body_all().await?, "user2");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn group(&mut self, path: &'static str) -> FibraResult<&mut Fibra> {
        Ok(self.route(path, Fibra::new())?.treat::<Fibra>().unwrap_or_else(|| unreachable!()))
    }

    /// Mount a handler
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    ///
    /// app.mount(addon::Logger{});
    /// app.mount(addon::Logger{});
    ///
    /// assert_eq!(app.handlers().len(), 2);
    /// ```
    pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
        self.mounted.push(Box::new(handler));
        self.mounted.last_mut().and_then(|h| h.as_handler_mut::<T>()).unwrap_or_else(|| unreachable!())
    }

    /// Add filters
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::Arc;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut app = Fibra::new();
    ///
    ///     app.get("/api/v1/user", "user1")?;
    ///     app.get("/api/v2/user", "user2")?;
    ///
    ///     app.limit().subdomain("api"); // domain name must begin with 'api'
    ///
    ///     let app = Arc::new(app);
    ///     let con = Arc::new(Connection::default());
    ///
    ///     // mock a request with incorrect subdomain
    ///     {
    ///         let req = Request::default().uri("http://app.example.com/api/v2/user");
    ///         let mut ctx = Context::from((app.clone(), con.clone(), req));
    ///
    ///         assert_eq!(ctx.next().await?.status_ref(), &Status::NOT_FOUND);
    ///     }
    ///
    ///     // mock a request with correct subdomain
    ///     {
    ///         let req = Request::default().uri("http://api.example.com/api/v2/user");
    ///         let mut ctx = Context::from((app.clone(), con.clone(), req));
    ///         let mut res = ctx.next().await?;
    ///
    ///         assert_eq!(res.status_ref(), &Status::OK);
    ///         assert_eq!(res.body_all().await?, "user2");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn limit(&mut self) -> &mut Limiter {
        self.limiter.get_or_insert(Limiter::default())
    }

    /// Set custom error handler
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::Arc;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut app = Fibra::new();
    ///
    ///     app.get("/api/v1/user", "user1")?;
    ///     app.get("/api/v2/user", "user2")?;
    ///
    ///     app.catch(|_, err| match err {
    ///         FibraError::PathNotFound(path) => (Status::NOT_FOUND, path).into(),
    ///         _ => Status::SERVICE_UNAVAILABLE.into(),
    ///     });
    ///
    ///     // mock a real request
    ///     let con = Connection::default();
    ///     let req = Request::default().uri("http://example.com/api/v3/user");
    ///     let mut ctx = Context::from((Arc::new(app), Arc::new(con), req));
    ///     let mut res = ctx.next().await?;
    ///
    ///     assert_eq!(res.status_ref(), &Status::NOT_FOUND);
    ///     assert_eq!(res.body_all().await?, "/api/v3/user");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn catch<F>(&mut self, f: F) -> &mut Catcher where F: Fn(&Catcher, FibraError) -> Response + Send + Sync + 'static {
        let catcher = self.catcher.get_or_insert(Catcher::default());
        catcher.custom(f);
        catcher
    }

    pub fn config(&mut self) {
        todo!()
    }

    /// Ensure the last item is type T, otherwise create it
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    ///
    /// app.ensure::<addon::Logger>(); // add a new handler
    /// app.ensure::<addon::Logger>(); // no effect because the last item is already a logger
    ///
    /// assert_eq!(app.handlers().len(), 1);
    /// ```
    pub fn ensure<T: Handler + Default>(&mut self) -> &mut T {
        if self.mounted.last().and_then(|h| h.as_handler::<T>()).is_none() {
            return self.mount(T::default());
        }

        self.mounted.last_mut().and_then(|h| h.as_handler_mut::<T>()).unwrap_or_else(|| unreachable!())
    }

    /// Get the mounted handlers
    pub fn handlers(&self) -> &Vec<BoxHandler> {
        &self.mounted
    }

    /// Bind tcp listener to a special address, we support calling this multiple times to listening on multiple addresses
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    ///
    /// assert_eq!(app.bind("0.0.0.0:0").is_ok(), true); // first random port
    /// assert_eq!(app.bind("0.0.0.0:0").is_ok(), true); // second random port
    /// assert_eq!(app.bind("0.0.0.0:65536").is_ok(), false); // invalid port
    /// ```
    pub fn bind(&mut self, addr: impl ToSocketAddrs) -> FibraResult<&mut Socket> {
        let last = self.sockets.len();
        self.sockets.push(StdTcpListener::bind(addr)?.into());
        Ok(&mut self.sockets[last])
    }

    /// Run the server, check the examples folder to see its usage
    pub async fn run(mut self) -> FibraResult<()> {
        use hyper::Server;
        use hyper::server::conn::AddrStream;
        use hyper::service::{make_service_fn, service_fn};
        
        // root router must have a catcher
        self.catcher.get_or_insert(Catcher::default());

        let mut sockets = std::mem::take(&mut self.sockets);
        let app = Arc::new(self);
        let srv = make_service_fn(|conn: &AddrStream| {
            let app = app.clone();
            let con = Arc::new(Connection::from((conn.local_addr(), conn.remote_addr())));

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    // simply incr because no concurrent requests on one connection
                    con.count_ref().fetch_add(1, atomic::Ordering::Relaxed);

                    // construct our own context object for each request
                    let ctx = Context::from((app.clone(), con.clone(), Request::from(req)));

                    // processing the request from the ctx's next method
                    async move { Ok::<_, FibraError>(ctx.next().await?.into()) }
                }))
            }
        });

        let mut servers = vec![];

        while let Some(socket) = sockets.pop() {
            servers.push(Server::from_tcp(socket.into())?.serve(srv));
        }

        futures::future::join_all(servers).await;

        Ok(())
    }
}

impl Default for Fibra {
    fn default() -> Self {
        Self { limiter: None, catcher: None, mounted: vec![], sockets: vec![] }
    }
}

#[async_trait]
impl Handler for Fibra {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        // todo catcher & limiter
        // self.mounted.handle(ctx).await
        todo!()
    }

    fn select(&self, idx: usize) -> Option<&BoxHandler> {
        // match idx == 0 {
        //     true => ,
        //     false => {}
        // }
        todo!()
    }
}
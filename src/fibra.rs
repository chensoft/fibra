use crate::route::*;
use crate::types::*;

pub struct Fibra {
    mounted: Vec<BoxHandler>,
    sockets: Vec<Socket>,
}

impl Fibra {
    /// Create a default app
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

    /// Register a route for any method
    pub fn all(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler)?;
        Ok(self)
    }

    /// Register a route without predefined method
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut app = Fibra::new();
    ///     let len = app.handlers().len();
    ///
    ///     app.route("/api/v1", "v1")?;
    ///     app.route("/api/v2", "v2")?;
    ///
    ///     assert_eq!(app.handlers().len(), len + 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn route(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Routine> {
        self.ensure::<Matcher>().insert(path, handler)
    }

    /// Register a subrouter with a prefix path
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
    ///     let con = Connection::default();
    ///     let mut ctx = Context::from((Arc::new(app), Arc::new(con), Request::default().uri("http://example.com/api/v2/user")));
    ///
    ///     assert_eq!(body::to_bytes(ctx.next().await?.body_mut()).await?, "user2");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn group(&mut self, path: &'static str) -> FibraResult<&mut Fibra> {
        Ok(self.route(path, Fibra::new())?.treat::<Fibra>().unwrap_or_else(|| unreachable!()))
    }

    /// Mount a handler to the app
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// let len = app.handlers().len();
    ///
    /// app.mount(addon::Logger{});
    /// app.mount(addon::Logger{});
    ///
    /// assert_eq!(app.handlers().len(), len + 2);
    /// ```
    pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
        self.mounted.push(Box::new(handler));
        self.mounted.last_mut().and_then(|h| h.as_handler_mut::<T>()).unwrap_or_else(|| unreachable!())
    }

    /// Set custom error handler
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// let catcher = app.catch(|_obj, err| {
    ///     match err {
    ///         FibraError::PanicError(_) => Status::SERVICE_UNAVAILABLE.into(),
    ///         _ => Status::INTERNAL_SERVER_ERROR.into(),
    ///     }
    /// });
    /// assert_eq!((catcher.custom)(catcher, FibraError::PanicError("panic".into())).status_ref(), &Status::SERVICE_UNAVAILABLE);
    /// ```
    pub fn catch<F>(&mut self, f: F) -> &mut Catcher where F: Fn(&Catcher, FibraError) -> Response + Send + Sync + 'static {
        let catcher = self.mounted.first_mut().and_then(|h| h.as_handler_mut::<Catcher>()).unwrap_or_else(|| unreachable!());
        catcher.custom(f);
        catcher
    }

    // todo do not impl from handler, just plain sync methods and field in fibra
    pub fn limit(&mut self) -> &mut Limiter {
        todo!()
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
    /// let len = app.handlers().len();
    ///
    /// app.ensure::<addon::Logger>(); // add a new handler
    /// app.ensure::<addon::Logger>(); // no effect because the last item is already a logger
    ///
    /// assert_eq!(app.handlers().len(), len + 1);
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
        Self { mounted: vec![Box::new(Catcher::default())], sockets: vec![] }
    }
}

#[async_trait]
impl Handler for Fibra {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
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
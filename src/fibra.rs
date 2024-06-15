//! The Router
use crate::route::*;
use crate::types::*;

/// Fibra, the core struct in this crate, acts as the central processor, handling and directing
/// all routing logic and operations. It processes incoming requests, matching them to predefined
/// handlers. Fibra also handles middlewares and errors, making it essential for building flexible
/// and robust web apps.
#[derive(Default)]
pub struct Fibra {
    /// Initial is used to store the prefix or base path of the current router. It represents the
    /// initial or starting path segment that all routes within this router should have in common.
    initial: Bytes,

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
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a route for GET method
    #[inline]
    pub fn get(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let routine = self.route(path, handler)?;
        routine.limit().method(Method::GET);
        Ok(routine)
    }

    /// Register a route for POST method
    #[inline]
    pub fn post(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let routine = self.route(path, handler)?;
        routine.limit().method(Method::POST);
        Ok(routine)
    }

    /// Register a route for PUT method
    #[inline]
    pub fn put(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let routine = self.route(path, handler)?;
        routine.limit().method(Method::PUT);
        Ok(routine)
    }

    /// Register a route for DELETE method
    #[inline]
    pub fn delete(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let routine = self.route(path, handler)?;
        routine.limit().method(Method::DELETE);
        Ok(routine)
    }

    /// Register a route for PATCH method
    #[inline]
    pub fn patch(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let routine = self.route(path, handler)?;
        routine.limit().method(Method::PATCH);
        Ok(routine)
    }

    /// Register a route for all methods
    #[inline]
    pub fn all(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        self.route(path, handler)
    }

    /// Register a route
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut app = Fibra::new();
    ///
    ///     app.get("/api/v1/user", "user1")?;
    ///     app.get("/api/v2/user", "user2")?;
    ///
    ///     // mock a real request and check the response body
    ///     let req = Request::new().uri("http://localip.cc/api/v2/user");
    ///     let ctx = Context::from((app, req));
    ///
    ///     assert_eq!(ctx.next().await?.body_all().await?, "user2");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn route(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let mut path = path.into();

        if !self.initial.is_empty() {
            let last = path.iter().rposition(|&v| v != b'/').map(|v| v + 1).unwrap_or_else(|| 0);
            let mut data = BytesMut::with_capacity(self.initial.len() + path.len());

            data.extend(self.initial.as_ref());
            data.extend(path.slice(..last));

            path = data.freeze();
        }

        self.ensure::<Matcher>().insert(path, handler)
    }

    /// Register a subrouter
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut api = Fibra::new();
    ///
    ///     let v1 = api.group("/v1")?;
    ///     v1.get("/user", "user1")?;
    ///
    ///     let v2 = api.group("/v2")?;
    ///     v2.get("/user", "user2")?;
    ///
    ///     // mock a real request
    ///     let req = Request::new().uri("http://api.localip.cc/v2/user");
    ///     let ctx = Context::from((api, req));
    ///
    ///     assert_eq!(ctx.next().await?.body_all().await?, "user2");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn group(&mut self, prefix: impl Into<Bytes>) -> FibraResult<&mut Fibra> {
        let pre = prefix.into();
        let pos = pre.iter().rposition(|&v| v != b'/').map(|v| v + 1).unwrap_or_else(|| 0);

        let mut val = BytesMut::with_capacity(self.initial.len() + pre.len());

        val.extend(self.initial.as_ref());
        val.extend(pre.slice(..pos));

        let sub = self.mount(Fibra::new());
        sub.initial = val.freeze();

        Ok(sub)
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
    /// app.mount(addon::ReqID::new());
    /// app.mount(addon::Logger::new());
    ///
    /// assert_eq!(app.handlers().len(), 2);
    /// ```
    #[inline]
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
    ///     app.get("/v1/user", "user1")?;
    ///     app.get("/v2/user", "user2")?;
    ///
    ///     app.limit().subdomain("api"); // domain name must begin with 'api'
    ///
    ///     let app = Arc::new(app);
    ///     let con = Arc::new(Connection::new());
    ///
    ///     // mock a request with incorrect subdomain
    ///     {
    ///         let req = Request::new().uri("http://app.localip.cc/v2/user");
    ///         let ctx = Context::new(app.clone(), con.clone(), req);
    ///
    ///         assert_eq!(ctx.next().await?.status_ref(), &Status::NOT_FOUND);
    ///     }
    ///
    ///     // mock a request with correct subdomain
    ///     {
    ///         let req = Request::new().uri("http://api.localip.cc/v2/user");
    ///         let ctx = Context::new(app, con, req);
    ///         let mut res = ctx.next().await?;
    ///
    ///         assert_eq!(res.status_ref(), &Status::OK);
    ///         assert_eq!(res.body_all().await?, "user2");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn limit(&mut self) -> &mut Limiter {
        self.limiter.get_or_insert(Limiter::new())
    }

    /// Handle failure responses and errors
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut app = Fibra::new();
    ///
    ///     app.get("/api/v1/user", "user1")?;
    ///     app.get("/api/v2/user", "user2")?;
    ///
    ///     app.catch(|res, err| {
    ///         res.body("Oops! User not found.")
    ///     });
    ///
    ///     // mock a real request
    ///     let ctx = Context::from((app, Request::new().uri("http://localip.cc/api/v3/user")));
    ///     let res = ctx.next().await?;
    ///
    ///     assert_eq!(res.status_ref(), &Status::NOT_FOUND);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn catch<F>(&mut self, f: F) -> &mut Self where F: Fn(Response, Option<FibraError>) -> Response + Send + Sync + 'static {
        self.catcher = Some(f.into());
        self
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
    #[inline]
    pub fn ensure<T: Handler + Default>(&mut self) -> &mut T {
        if self.mounted.last().and_then(|h| h.as_handler::<T>()).is_none() {
            return self.mount(T::default());
        }

        self.mounted.last_mut().and_then(|h| h.as_handler_mut::<T>()).unwrap_or_else(|| unreachable!())
    }

    /// Get the mounted handlers
    #[inline]
    pub fn handlers(&self) -> &Vec<BoxHandler> {
        &self.mounted
    }

    /// Bind tcp listener to a local address, calling this multiple times to listening on multiple addresses
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use socket2::{Socket, Domain, Type, Protocol};
    ///
    /// let mut app = Fibra::new();
    ///
    /// // 0 means random port
    /// assert_eq!(app.bind(0).is_ok(), true);           // dual-stack, v4 & v6
    /// assert_eq!(app.bind(":0").is_ok(), true);        // dual-stack, v4 & v6
    /// assert_eq!(app.bind("[::]:0").is_ok(), true);    // dual-stack, v4 & v6
    /// assert_eq!(app.bind("[::1]:0").is_ok(), true);   // ipv6-only
    /// assert_eq!(app.bind("0.0.0.0:0").is_ok(), true); // ipv4-only
    /// assert_eq!(app.bind(Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))).is_ok(), true); // raw socket
    /// ```
    #[inline]
    pub fn bind(&mut self, addr: impl TryIntoListener) -> FibraResult<&mut Socket> {
        self.sockets.push(addr.try_into_listener()?);
        Ok(self.sockets.last_mut().unwrap_or_else(|| unreachable!()))
    }

    /// Run the server, check the examples folder to see its usage
    pub async fn run(mut self) -> FibraResult<()> {
        use hyper_util::server::conn::auto::Builder;
        use hyper::service::service_fn;

        // root router must have a catcher
        self.catcher.get_or_insert(Catcher::new());

        // create service handler to serve
        let sockets = std::mem::take(&mut self.sockets);
        let mut servers = vec![];

        let app = Arc::new(self);
        let svc = |app: Arc<Fibra>, io: TokioIo<TcpStream>| {
            let con = Arc::new(Connection::from((io.inner().local_addr().unwrap(), io.inner().peer_addr().unwrap())));

            tokio::task::spawn(async move {
                Builder::new(hyper_util::rt::TokioExecutor::new()).serve_connection(io, service_fn(|req: hyper::Request<hyper::body::Incoming>| {
                    // construct our own context object for each request
                    let ctx = Context::new(app.clone(), con.clone(), Request::from(req));

                    // processing the request from the ctx's next method
                    async move { Ok::<_, FibraError>(ctx.next().await?.into()) }
                })).await.unwrap();
            });
        };

        for socket in sockets {
            let tcp = AsyncTcpListener::from_std(socket.into())?;
            let srv = {
                let app = app.clone();

                async move {
                    loop {
                        let (con, _) = match tcp.accept().await {
                            Ok(obj) => obj,
                            Err(_) => continue,
                        };

                        svc(app.clone(), TokioIo::new(con));
                    }
                }
            };

            servers.push(srv);
        }

        futures::future::join_all(servers).await;

        Ok(())
    }
}

#[async_trait]
impl Handler for Fibra {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        // match the beginning segment
        if !ctx.path().as_bytes().starts_with(self.initial.as_ref()) {
            return ctx.next().await;
        }

        // block requests that fail the test
        if let Some(limiter) = &self.limiter {
            if !limiter.test(&ctx) {
                return ctx.next().await;
            }
        }

        // the root router and subrouters with a Catcher will handle errors here
        if let Some(catcher) = &self.catcher {
            return Ok(catcher.protect(self.mounted.handle(ctx)).await);
        }

        // subrouters without a Catcher will handle requests here. If an error occurs, it will
        // propagate up to the nearest parent that has a Catcher to handle it.
        self.mounted.handle(ctx).await
    }
}
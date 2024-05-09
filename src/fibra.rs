use crate::addon;
use crate::route::*;
use crate::types::*;

pub struct Fibra {
    mounted: Vec<BoxHandler>,
    sockets: Vec<Socket>,
}

impl Fibra {
    // todo static check path is valid and remove fibraresult
    pub fn get(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler, Some(Method::GET))
    }

    pub fn post(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler, Some(Method::POST))
    }

    pub fn put(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler, Some(Method::PUT))
    }

    pub fn delete(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler, Some(Method::DELETE))
    }

    pub fn patch(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler, Some(Method::PATCH))
    }

    pub fn all(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Self> {
        self.route(path, handler, None)
    }

    pub fn route(&mut self, path: &'static str, handler: impl Handler, method: Option<Method>) -> FibraResult<&mut Self> {
        self.ensure::<Router>().add(path, handler, method)?;
        Ok(self)
    }

    pub fn group(&mut self, path: &'static str) -> FibraResult<&mut Fibra> {
        self.route(path, Fibra::default(), None)
    }

    /// Set custom error handler
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::default();
    /// let catcher = app.catch(|_obj, _err| Status::SERVICE_UNAVAILABLE.into());
    /// assert_eq!((catcher.custom)(catcher, FibraError::PanicError("panic".into())).status_ref(), &Status::SERVICE_UNAVAILABLE);
    /// ```
    pub fn catch(&mut self, f: impl Fn(&addon::Catcher, FibraError) -> Response + Send + Sync + 'static) -> &mut addon::Catcher {
        let catcher = self.mounted.first_mut().and_then(|h| h.as_handler_mut::<addon::Catcher>()).unwrap_or_else(|| unreachable!());
        catcher.custom = Box::new(f);
        catcher
    }

    // pub fn filter(&mut self) -> &mut Filter {
    //     self.force()
    // }

    /// Mount a handler to the app
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::default();
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

    /// Ensure the last item is type T, otherwise create it
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::default();
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
    /// let mut app = Fibra::default();
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
        Self { mounted: vec![Box::new(addon::Catcher::default())], sockets: vec![] }
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
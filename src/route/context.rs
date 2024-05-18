//! Request Context
use crate::route::*;
use crate::types::*;
use crate::fibra::*;

/// Context which holds the connection and request
pub struct Context {
    /// The root app instance
    app: Arc<Fibra>,

    /// Current connection ref
    conn: Arc<Connection>,

    /// The served requests of the connection, begins from 1
    served: usize,

    /// Current request object
    req: Request,

    /// The named params after path matching
    params: IndexMap<String, String>,

    /// The query string of the Uri
    queries: IndexMap<String, String>,

    /// Internal routing stack
    routing: Vec<*const dyn Handler>,

    // request path to be matched
    matchable: Bytes,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    /// The root app instance
    pub fn app(&self) -> &Arc<Fibra> {
        &self.app
    }

    /// Current connection, multiple requests may belong to one connection
    pub fn conn(&self) -> &Arc<Connection> {
        &self.conn
    }

    /// Current connection's unique id
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Context::default().connid() > 0, true);
    /// ```
    pub fn connid(&self) -> u128 {
        *self.conn.id_ref()
    }

    /// Current connection's established time
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// let old = SystemTime::now();
    /// let ctx = Context::default();
    ///
    /// assert_eq!(ctx.established() >= &old, true);
    /// assert_eq!(ctx.established() <= &SystemTime::now(), true);
    /// ```
    pub fn established(&self) -> &SystemTime {
        self.conn.created_ref()
    }

    /// The number of requests served by the connection until this request
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Context::default().served(), 1);
    /// ```
    pub fn served(&self) -> usize {
        self.served
    }

    /// The endpoint on the local machine for the connection
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// let con = Connection::from((SocketAddr::from(([127, 0, 0, 1], 3000)), SocketAddr::from(([8, 8, 8, 8], 80))));
    /// let ctx = Context::from(con);
    ///
    /// assert_eq!(ctx.local().to_string(), "127.0.0.1:3000");
    /// assert_eq!(ctx.remote().to_string(), "8.8.8.8:80");
    /// ```
    pub fn local(&self) -> &SocketAddr {
        self.conn.sockaddr_ref()
    }

    /// The remote address that the connection comes from
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// let con = Connection::from((SocketAddr::from(([127, 0, 0, 1], 3000)), SocketAddr::from(([8, 8, 8, 8], 80))));
    /// let ctx = Context::from(con);
    ///
    /// assert_eq!(ctx.local().to_string(), "127.0.0.1:3000");
    /// assert_eq!(ctx.remote().to_string(), "8.8.8.8:80");
    /// ```
    pub fn remote(&self) -> &SocketAddr {
        self.conn.peeraddr_ref()
    }
}

impl Context {
    /// Current request object
    pub fn req(&self) -> &Request {
        &self.req
    }

    /// Request's unique id
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::default();
    ///
    /// assert_eq!(ctx.reqid() > 0, true);
    /// assert_ne!(ctx.reqid(), Context::default().reqid()); // reqid is unique
    /// ```
    pub fn reqid(&self) -> u128 {
        *self.req.id_ref()
    }

    /// Request's created time
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// let old = SystemTime::now();
    /// let ctx = Context::default();
    ///
    /// assert_eq!(ctx.created() >= &old, true);
    /// assert_eq!(ctx.created() <= &SystemTime::now(), true);
    /// ```
    pub fn created(&self) -> &SystemTime {
        self.req.created_ref()
    }

    /// Request's method
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().method(Method::PUT));
    ///
    /// assert_eq!(ctx.method(), &Method::PUT);
    /// assert_eq!(ctx.is_get(), false);
    /// assert_eq!(ctx.is_post(), false);
    /// assert_eq!(ctx.is_put(), true);
    /// assert_eq!(ctx.is_delete(), false);
    /// assert_eq!(ctx.is_patch(), false);
    /// ```
    pub fn method(&self) -> &Method {
        self.req.method_ref()
    }

    /// Check method
    pub fn is_get(&self) -> bool {
        self.method() == Method::GET
    }

    /// Check method
    pub fn is_post(&self) -> bool {
        self.method() == Method::POST
    }

    /// Check method
    pub fn is_put(&self) -> bool {
        self.method() == Method::PUT
    }

    /// Check method
    pub fn is_delete(&self) -> bool {
        self.method() == Method::DELETE
    }

    /// Check method
    pub fn is_head(&self) -> bool {
        self.method() == Method::HEAD
    }

    /// Check method
    pub fn is_options(&self) -> bool {
        self.method() == Method::OPTIONS
    }

    /// Check method
    pub fn is_connect(&self) -> bool {
        self.method() == Method::CONNECT
    }

    /// Check method
    pub fn is_patch(&self) -> bool {
        self.method() == Method::PATCH
    }

    /// Check method
    pub fn is_trace(&self) -> bool {
        self.method() == Method::TRACE
    }

    /// Request's uri
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://localip.cc"));
    ///
    /// assert_eq!(ctx.uri(), "http://localip.cc/");
    /// ```
    pub fn uri(&self) -> &Uri {
        self.req.uri_ref()
    }

    /// Request's scheme
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("https://localip.cc"));
    ///
    /// assert_eq!(ctx.scheme(), &Scheme::HTTPS);
    /// assert_eq!(ctx.is_secure(), true);
    /// ```
    pub fn scheme(&self) -> &Scheme {
        self.req.scheme()
    }

    /// Check the scheme
    pub fn is_secure(&self) -> bool {
        self.req.scheme() == &Scheme::HTTPS
    }

    /// Request's authority
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://user:pass@localip.cc"));
    ///
    /// assert_eq!(ctx.authority(), Some(&Authority::from_static("user:pass@localip.cc")));
    /// ```
    pub fn authority(&self) -> Option<&Authority> {
        self.req.authority()
    }

    /// Request's domain
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://user:pass@localip.cc"));
    ///
    /// assert_eq!(ctx.domain(), "localip.cc");
    /// ```
    pub fn domain(&self) -> &str {
        self.req.domain()
    }

    /// Request's subdomain
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://user:pass@git.localip.cc"));
    ///
    /// assert_eq!(ctx.subdomain(), "git");
    /// ```
    pub fn subdomain(&self) -> &str {
        self.req.subdomain()
    }

    /// Request's host
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://user:pass@git.localip.cc"));
    ///
    /// assert_eq!(ctx.host(), "git.localip.cc");
    /// ```
    pub fn host(&self) -> &str {
        self.req.host()
    }

    /// Request's port
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://localip.cc:3000"));
    ///
    /// assert_eq!(ctx.port(), 3000);
    /// ```
    pub fn port(&self) -> u16 {
        self.req.port()
    }

    /// Request's path
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://localip.cc/repo/fibra"));
    ///
    /// assert_eq!(ctx.path(), "/repo/fibra");
    /// ```
    pub fn path(&self) -> &str {
        self.req.path()
    }

    /// Request's query value
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://localip.cc/?foo=bar"));
    ///
    /// assert_eq!(ctx.query("foo"), "bar");
    /// assert_eq!(ctx.query("key"), "");
    /// ```
    pub fn query(&self, key: &str) -> &str {
        self.queries.get(key).map(|v| v.as_str()).unwrap_or("")
    }

    /// Request's query string
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://localip.cc/?foo=bar&key=%E4%BD%A0%E5%A5%BD"));
    ///
    /// assert_eq!(ctx.query("foo"), "bar");
    /// assert_eq!(ctx.query("key"), "你好"); // url decoded
    /// assert_eq!(ctx.queries(), &indexmap::indexmap! { "foo".to_string() => "bar".to_string(), "key".to_string() => "你好".to_string() });
    /// ```
    pub fn queries(&self) -> &IndexMap<String, String> {
        &self.queries
    }

    /// Request's whole uri
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().uri("http://user:pass@git.localip.cc/repo/fibra?foo=bar&key=%E4%BD%A0%E5%A5%BD"));
    ///
    /// assert_eq!(ctx.href(), "http://user:pass@git.localip.cc/repo/fibra?foo=bar&key=%E4%BD%A0%E5%A5%BD".to_string());
    /// ```
    pub fn href(&self) -> String {
        self.req.href()
    }

    /// Request's version
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().version(Version::HTTP_10));
    ///
    /// assert_eq!(ctx.version(), &Version::HTTP_10);
    /// assert_eq!(ctx.is_http10(), true);
    /// assert_eq!(ctx.is_http11(), false);
    /// assert_eq!(ctx.is_http1x(), true);
    /// assert_eq!(ctx.is_http2(), false);
    /// ```
    pub fn version(&self) -> &Version {
        self.req.version_ref()
    }

    /// Check http version
    pub fn is_http1x(&self) -> bool {
        self.is_http10() || self.is_http11()
    }

    /// Check http version
    pub fn is_http10(&self) -> bool {
        self.version() == &Version::HTTP_10
    }

    /// Check http version
    pub fn is_http11(&self) -> bool {
        self.version() == &Version::HTTP_11
    }

    /// Check http version
    pub fn is_http2(&self) -> bool {
        self.version() == &Version::HTTP_2
    }

    /// Request's header value
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().header("content-type", "application/json").header("cache-control", "no-cache"));
    ///
    /// assert_eq!(ctx.header("content-type").map(|v| v.as_bytes()), Some("application/json".as_bytes()));
    /// assert_eq!(ctx.header("cache-control").map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// assert_eq!(ctx.header("accept-encoding"), None);
    /// ```
    pub fn header(&self, key: impl AsHeaderName) -> Option<&HeaderValue> {
        self.req.header_ref(key)
    }

    /// Request's headers
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::default().header("content-type", "application/json").header("cache-control", "no-cache"));
    ///
    /// let headers = ctx.headers();
    ///
    /// assert_eq!(headers.get("content-type").map(|v| v.as_bytes()), Some("application/json".as_bytes()));
    /// assert_eq!(headers.get("cache-control").map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// assert_eq!(headers.get("accept-encoding"), None);
    /// ```
    pub fn headers(&self) -> &HeaderMap {
        self.req.headers_ref()
    }

    /// Named params after matching
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let arg = indexmap::indexmap! {
    ///     String::from("name") => String::from("user1"),
    ///     String::from("code") => String::from("12345"),
    /// };
    /// let mut ctx = Context::default();
    /// ctx.params_mut().extend(arg);
    ///
    /// assert_eq!(ctx.param(""), "");
    /// assert_eq!(ctx.param("name"), "user1");
    /// assert_eq!(ctx.param("code"), "12345");
    /// assert_eq!(ctx.param("none"), "");
    /// ```
    pub fn param(&self, key: &str) -> &str {
        self.params.get(key).map(|v| v.as_str()).unwrap_or("")
    }

    /// Named params after matching
    pub fn params(&self) -> &IndexMap<String, String> {
        &self.params
    }

    /// Named params after matching
    pub fn params_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl Context {
    /// Read all body contents into a BufList
    pub async fn read_all(&mut self) -> BufList {
        let mut list = BufList::new();
        while let Some(bytes) = self.read_chunk().await {
            list.push_chunk(bytes);
        }
        list
    }

    /// Read one chunk into a Bytes
    pub async fn read_chunk(&mut self) -> Option<Bytes> {
        use body::HttpBody;
        self.req.body_mut().data().await.and_then(|r| r.ok())
    }
}

impl Context {
    /// Reject current request with FORBIDDEN by default
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     assert_eq!(Context::default().reject(None)?.status_ref(), &Status::FORBIDDEN);
    ///     assert_eq!(Context::default().reject(Some(Status::BAD_REQUEST))?.status_ref(), &Status::BAD_REQUEST);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn reject(self, status: Option<Status>) -> FibraResult<Response> {
        Ok(status.unwrap_or(Status::FORBIDDEN).into())
    }

    /// Rewrite the current request with a different URI and Body, and re-handle the request transparently
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let app = fibra! { 
    ///         get("/v1") => "v1",
    ///         get("/v2") => "v2",
    ///     };
    ///     let ctx = Context::from((app, Request::default().uri("http://localip.cc/v1")));
    ///
    ///     assert_eq!(ctx.rewrite("http://localip.cc/v2", "").await?.body_all().await?, "v2");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn rewrite(self, to: impl IntoUri, body: impl Into<Body>) -> FibraResult<Response> {
        let ctx = Context::from((self.app, self.conn, self.req.uri(to).body(body)));
        ctx.next().await
    }

    /// Redirect current request with FOUND by default
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     assert_eq!(Context::default().redirect("http://localip.cc", None)?.status_ref(), &Status::FOUND);
    ///     assert_eq!(Context::default().redirect("http://localip.cc", None)?.header_ref(header::LOCATION), Some(&HeaderValue::from_static("http://localip.cc/")));
    ///     assert_eq!(Context::default().redirect("http://localip.cc", Some(Status::MOVED_PERMANENTLY))?.status_ref(), &Status::MOVED_PERMANENTLY);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn redirect(self, to: impl IntoUri, status: Option<Status>) -> FibraResult<Response> {
        let location = HeaderValue::try_from(to.into_uri().to_string())?;
        let redirect = status.unwrap_or(Status::FOUND);
        Ok(Response::default().status(redirect).header(header::LOCATION, location))
    }

    /// Find the next handler and execute it
    pub async fn next(mut self) -> FibraResult<Response> {
        if let Some(handler) = self.routing.pop() {
            return unsafe { &*handler }.handle(self).await;
        }

        Err(FibraError::PathNotFound)
    }

    /// todo
    #[inline]
    pub fn push(&mut self, cur: *const dyn Handler) {
        self.routing.push(cur);
    }
    
    pub fn matchable(&self) {
        todo!()
    }
}

/// Construct from client request
impl From<(Arc<Fibra>, Arc<Connection>, Request)> for Context {
    fn from((app, conn, req): (Arc<Fibra>, Arc<Connection>, Request)) -> Self {
        let served = conn.count_add(1);
        let queries = form_urlencoded::parse(req.query().as_bytes()).into_owned().collect();
        let matchable = Bytes::from(req.path().to_string());

        let mut myself = Self { app, conn, served, req, params: IndexMap::new(), queries, routing: vec![], matchable };
        myself.push(myself.app().as_ref());
        myself
    }
}

/// FOR MOCK USE ONLY
impl Default for Context {
    fn default() -> Self {
        (Arc::new(Fibra::default()), Arc::new(Connection::default()), Request::default()).into()
    }
}

/// FOR MOCK USE ONLY
impl From<Fibra> for Context {
    fn from(app: Fibra) -> Self {
        (Arc::new(app), Arc::new(Connection::default()), Request::default()).into()
    }
}

/// FOR MOCK USE ONLY
impl From<Connection> for Context {
    fn from(con: Connection) -> Self {
        (Arc::new(Fibra::default()), Arc::new(con), Request::default()).into()
    }
}

/// FOR MOCK USE ONLY
impl From<Request> for Context {
    fn from(req: Request) -> Self {
        (Arc::new(Fibra::default()), Arc::new(Connection::default()), req).into()
    }
}

/// FOR MOCK USE ONLY
impl From<(Fibra, Request)> for Context {
    fn from((app, req): (Fibra, Request)) -> Self {
        (Arc::new(app), Arc::new(Connection::default()), req).into()
    }
}

/// FOR MOCK USE ONLY
impl From<(Fibra, Connection, Request)> for Context {
    fn from((app, con, req): (Fibra, Connection, Request)) -> Self {
        (Arc::new(app), Arc::new(con), req).into()
    }
}
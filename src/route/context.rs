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
    queries: OnceCell<IndexMap<String, String>>,

    /// Internal routing stack, service is the parent, vector is whether it's a vector, index is the index of children
    routing: Vec<(*const dyn Service, bool, usize)>, // (service, index, vector)
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    /// Construct from client request
    #[inline]
    pub fn new(app: Arc<Fibra>, conn: Arc<Connection>, req: Request) -> Self {
        let served = conn.count_add(1);

        let mut myself = Self { app, conn, served, req, params: IndexMap::new(), queries: OnceCell::new(), routing: vec![] };
        myself.push(myself.app().as_ref(), false, 0);
        myself
    }

    /// The root app instance
    #[inline]
    pub fn app(&self) -> &Arc<Fibra> {
        &self.app
    }

    /// Current connection, multiple requests may belong to one connection
    #[inline]
    pub fn conn(&self) -> &Arc<Connection> {
        &self.conn
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
    #[inline]
    pub fn established(&self) -> &SystemTime {
        self.conn.created_ref()
    }

    /// The number of requests served by the connection until this request
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::Arc;
    ///
    /// let con = Arc::new(Connection::new());
    ///
    /// assert_eq!(Context::from(con.clone()).served(), 1);
    /// assert_eq!(Context::from(con.clone()).served(), 2);
    /// assert_eq!(Context::from(con.clone()).served(), 3);
    /// ```
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn remote(&self) -> &SocketAddr {
        self.conn.peeraddr_ref()
    }

    /// todo The remote ip address
    #[inline]
    pub fn realip(&self) -> IpAddr {
        self.remote().ip()
    }
}

impl Context {
    /// Current request object
    #[inline]
    pub fn req(&self) -> &Request {
        &self.req
    }

    /// For developer use only
    #[inline]
    pub fn req_mut(&mut self) -> &mut Request {
        &mut self.req
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
    #[inline]
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
    /// let ctx = Context::from(Request::new().method(Method::PUT));
    ///
    /// assert_eq!(ctx.method(), &Method::PUT);
    /// assert_eq!(ctx.is_get(), false);
    /// assert_eq!(ctx.is_post(), false);
    /// assert_eq!(ctx.is_put(), true);
    /// assert_eq!(ctx.is_delete(), false);
    /// assert_eq!(ctx.is_patch(), false);
    /// ```
    #[inline]
    pub fn method(&self) -> &Method {
        self.req.method_ref()
    }

    /// Check method
    #[inline]
    pub fn is_get(&self) -> bool {
        self.method() == Method::GET
    }

    /// Check method
    #[inline]
    pub fn is_post(&self) -> bool {
        self.method() == Method::POST
    }

    /// Check method
    #[inline]
    pub fn is_put(&self) -> bool {
        self.method() == Method::PUT
    }

    /// Check method
    #[inline]
    pub fn is_delete(&self) -> bool {
        self.method() == Method::DELETE
    }

    /// Check method
    #[inline]
    pub fn is_head(&self) -> bool {
        self.method() == Method::HEAD
    }

    /// Check method
    #[inline]
    pub fn is_options(&self) -> bool {
        self.method() == Method::OPTIONS
    }

    /// Check method
    #[inline]
    pub fn is_connect(&self) -> bool {
        self.method() == Method::CONNECT
    }

    /// Check method
    #[inline]
    pub fn is_patch(&self) -> bool {
        self.method() == Method::PATCH
    }

    /// Check method
    #[inline]
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
    /// let ctx = Context::from(Request::new().uri("http://example.com"));
    ///
    /// assert_eq!(ctx.uri(), "http://example.com/");
    /// ```
    #[inline]
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
    /// let ctx = Context::from(Request::new().uri("https://example.com"));
    ///
    /// assert_eq!(ctx.scheme(), &Scheme::HTTPS);
    /// assert_eq!(ctx.is_secure(), true);
    /// ```
    #[inline]
    pub fn scheme(&self) -> &Scheme {
        self.req.scheme()
    }

    /// Check the scheme
    #[inline]
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
    /// let ctx = Context::from(Request::new().uri("http://user:pass@example.com"));
    ///
    /// assert_eq!(ctx.authority(), Some(&Authority::from_static("user:pass@example.com")));
    /// ```
    #[inline]
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
    /// let ctx = Context::from(Request::new().uri("http://user:pass@example.com"));
    ///
    /// assert_eq!(ctx.domain(), "example.com");
    /// ```
    #[inline]
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
    /// let ctx = Context::from(Request::new().uri("http://user:pass@git.example.com"));
    ///
    /// assert_eq!(ctx.subdomain(), "git");
    /// ```
    #[inline]
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
    /// assert_eq!(Context::from(Request::new().uri("http://user:pass@git.example.com")).host(), "git.example.com");
    /// assert_eq!(Context::from(Request::new().uri("http://user:pass@git.example.com:3000")).host(), "git.example.com:3000");
    /// ```
    #[inline]
    pub fn host(&self) -> &str {
        self.req.host()
    }

    /// Request's hostname
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Context::from(Request::new().uri("http://user:pass@git.example.com")).host(), "git.example.com");
    /// assert_eq!(Context::from(Request::new().uri("http://user:pass@git.example.com:3000")).host(), "git.example.com");
    /// ```
    #[inline]
    pub fn hostname(&self) -> &str {
        self.req.hostname()
    }

    /// Request's port
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::new().uri("http://example.com:3000"));
    ///
    /// assert_eq!(ctx.port(), 3000);
    /// ```
    #[inline]
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
    /// let ctx = Context::from(Request::new().uri("http://example.com/repo/fibra"));
    ///
    /// assert_eq!(ctx.path(), "/repo/fibra");
    /// ```
    #[inline]
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
    /// let ctx = Context::from(Request::new().uri("http://example.com/?foo=bar"));
    ///
    /// assert_eq!(ctx.query("foo"), "bar");
    /// assert_eq!(ctx.query("key"), "");
    /// ```
    #[inline]
    pub fn query(&self, key: &str) -> &str {
        self.queries().get(key).map(|v| v.as_str()).unwrap_or("")
    }

    /// Request's query string
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::new().uri("http://example.com/?foo=bar&key=%E4%BD%A0%E5%A5%BD"));
    ///
    /// assert_eq!(ctx.query("foo"), "bar");
    /// assert_eq!(ctx.query("key"), "你好"); // url decoded
    /// assert_eq!(ctx.queries(), &indexmap::indexmap! { "foo".to_string() => "bar".to_string(), "key".to_string() => "你好".to_string() });
    /// ```
    #[inline]
    pub fn queries(&self) -> &IndexMap<String, String> {
        self.queries.get_or_init(|| {
            form_urlencoded::parse(self.req.query().as_bytes()).into_owned().collect()
        })
    }

    /// Request's whole uri
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::new().uri("http://user:pass@git.example.com/repo/fibra?foo=bar&key=%E4%BD%A0%E5%A5%BD"));
    ///
    /// assert_eq!(ctx.href(), "http://user:pass@git.example.com/repo/fibra?foo=bar&key=%E4%BD%A0%E5%A5%BD".to_string());
    /// ```
    #[inline]
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
    /// let ctx = Context::from(Request::new().version(Version::HTTP_10));
    ///
    /// assert_eq!(ctx.version(), &Version::HTTP_10);
    /// assert_eq!(ctx.is_http10(), true);
    /// assert_eq!(ctx.is_http11(), false);
    /// assert_eq!(ctx.is_http1x(), true);
    /// assert_eq!(ctx.is_http2(), false);
    /// ```
    #[inline]
    pub fn version(&self) -> &Version {
        self.req.version_ref()
    }

    /// Check http version
    #[inline]
    pub fn is_http1x(&self) -> bool {
        self.is_http10() || self.is_http11()
    }

    /// Check http version
    #[inline]
    pub fn is_http10(&self) -> bool {
        self.version() == &Version::HTTP_10
    }

    /// Check http version
    #[inline]
    pub fn is_http11(&self) -> bool {
        self.version() == &Version::HTTP_11
    }

    /// Check http version
    #[inline]
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
    /// let ctx = Context::from(Request::new().header("content-type", "application/json").header("cache-control", "no-cache"));
    ///
    /// assert_eq!(ctx.header("content-type").map(|v| v.as_bytes()), Some("application/json".as_bytes()));
    /// assert_eq!(ctx.header("cache-control").map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// assert_eq!(ctx.header("accept-encoding"), None);
    /// ```
    #[inline]
    pub fn header(&self, key: impl AsHeaderName) -> &str {
        self.req.header_ref(key)
    }

    /// Request's headers
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let ctx = Context::from(Request::new().header("content-type", "application/json").header("cache-control", "no-cache"));
    ///
    /// let headers = ctx.headers();
    ///
    /// assert_eq!(headers.get("content-type").map(|v| v.as_bytes()), Some("application/json".as_bytes()));
    /// assert_eq!(headers.get("cache-control").map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// assert_eq!(headers.get("accept-encoding"), None);
    /// ```
    #[inline]
    pub fn headers(&self) -> &HeaderMap<String> {
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
    #[inline]
    pub fn param(&self, key: &str) -> &str {
        self.params.get(key).map(|v| v.as_str()).unwrap_or("")
    }

    /// Named params after matching
    #[inline]
    pub fn params(&self) -> &IndexMap<String, String> {
        &self.params
    }

    /// Named params after matching
    #[inline]
    pub fn params_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl Context {
    /// Read all body contents into a BufList
    #[inline]
    pub async fn read_all(&mut self) -> Option<Bytes> {
        self.req.body_mut().read_all().await
    }

    /// Read one frame into a Bytes
    #[inline]
    pub async fn read_frame(&mut self) -> Option<Bytes> {
        self.req.body_mut().read_frame().await
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
    #[inline]
    pub fn reject(self, status: Option<Status>) -> FibraResult<Response> {
        Ok(status.unwrap_or(Status::FORBIDDEN).into())
    }

    /// Rewrite the current request server-side without the client perceiving it
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
    ///     let ctx = Context::from((app, Request::new().uri("http://example.com/v1")));
    ///
    ///     assert_eq!(ctx.rewrite("http://example.com/v2", None).await?.body_all().await.unwrap_or_default(), "v2");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub async fn rewrite(mut self, to: impl AsRef<str>, body: Option<Body>) -> FibraResult<Response> {
        let body = match body {
            Some(val) => val,
            None => std::mem::take(self.req.body_mut()),
        };

        let ctx = Context::new(self.app, self.conn, self.req.uri(Uri::try_from(to.as_ref())?).body(body));
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
    ///     assert_eq!(Context::default().redirect("http://example.com", Redirect::TemporaryRedirect307)?.status_ref(), &Status::TEMPORARY_REDIRECT);
    ///     assert_eq!(Context::default().redirect("http://example.com", Redirect::TemporaryRedirect307)?.header_ref(header::LOCATION), Some(&HeaderValue::from_static("http://example.com/")));
    ///     assert_eq!(Context::default().redirect("http://example.com", Redirect::PermanentRedirect308)?.status_ref(), &Status::PERMANENT_REDIRECT);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn redirect(self, to: impl IntoUri, code: Redirect) -> FibraResult<Response> {
        Ok(Response::new().status(code).header(header::LOCATION, to.into_uri().to_string()))
    }

    /// Find the next service and execute it
    pub async fn next(mut self) -> FibraResult<Response> {
        while let Some((obj, vec, idx)) = self.routing.last_mut() {
            let cur: &dyn Service = unsafe { &**obj };

            // service itself
            if !*vec {
                self.routing.pop();
                return cur.invoke(self).await;
            }

            // child service
            if let Some(cld) = cur.select(*idx) {
                *idx += 1;
                return cld.invoke(self).await;
            }

            self.routing.pop();
        }

        Ok(Status::NOT_FOUND.into())
    }

    /// Push the nested group of services into stack
    #[inline]
    pub fn push(&mut self, obj: *const dyn Service, vec: bool, idx: usize) {
        self.routing.push((obj, vec, idx));
    }
}

/// FOR MOCK USE ONLY
impl Default for Context {
    #[inline]
    fn default() -> Self {
        Self::new(Arc::new(Fibra::new()), Arc::new(Connection::new()), Request::new())
    }
}

/// FOR MOCK USE ONLY
impl From<Fibra> for Context {
    #[inline]
    fn from(app: Fibra) -> Self {
        Self::new(Arc::new(app), Arc::new(Connection::new()), Request::new())
    }
}

/// FOR MOCK USE ONLY
impl From<(Fibra, Request)> for Context {
    #[inline]
    fn from((app, req): (Fibra, Request)) -> Self {
        Self::new(Arc::new(app), Arc::new(Connection::new()), req)
    }
}

/// FOR MOCK USE ONLY
impl From<(Fibra, Connection, Request)> for Context {
    #[inline]
    fn from((app, con, req): (Fibra, Connection, Request)) -> Self {
        Self::new(Arc::new(app), Arc::new(con), req)
    }
}

/// FOR MOCK USE ONLY
impl From<Connection> for Context {
    #[inline]
    fn from(con: Connection) -> Self {
        Self::new(Arc::new(Fibra::new()), Arc::new(con), Request::new())
    }
}

/// FOR MOCK USE ONLY
impl From<Arc<Connection>> for Context {
    #[inline]
    fn from(con: Arc<Connection>) -> Self {
        Self::new(Arc::new(Fibra::new()), con, Request::new())
    }
}

/// FOR MOCK USE ONLY
impl From<Request> for Context {
    #[inline]
    fn from(req: Request) -> Self {
        Self::new(Arc::new(Fibra::new()), Arc::new(Connection::new()), req)
    }
}
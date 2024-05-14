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
    routing: Vec<(*const dyn Handler, usize)>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    /// The root app instance
    pub fn app(&self) -> &Fibra {
        &self.app
    }

    /// Current connection, multiple requests may belong to one connection
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Current connection's unique id
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Context::from(Request::default()).connid() > 0, true);
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
    /// use chrono::Local;
    ///
    /// let old = Local::now();
    /// let ctx = Context::from(Request::default());
    ///
    /// assert_eq!(ctx.established() >= &old, true);
    /// assert_eq!(ctx.established() <= &Local::now(), true);
    /// ```
    pub fn established(&self) -> &DateTime<Local> {
        self.conn.created_ref()
    }

    /// The number of requests served by the connection until this request
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Context::from(Request::default()).served(), 1);
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
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    ///
    /// let app = Arc::new(Fibra::new());
    /// let con = Arc::new(Connection::from((SocketAddr::from(([127, 0, 0, 1], 3000)), SocketAddr::from(([8, 8, 8, 8], 80)))));
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
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
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    ///
    /// let app = Arc::new(Fibra::new());
    /// let con = Arc::new(Connection::from((SocketAddr::from(([127, 0, 0, 1], 3000)), SocketAddr::from(([8, 8, 8, 8], 80)))));
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
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
    /// let ctx = Context::from(Request::default());
    ///
    /// assert_eq!(ctx.reqid() > 0, true);
    /// assert_ne!(ctx.reqid(), Context::from(Request::default()).reqid()); // reqid is unique
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
    /// use chrono::Local;
    ///
    /// let old = Local::now();
    /// let ctx = Context::from(Request::default());
    ///
    /// assert_eq!(ctx.created() >= &old, true);
    /// assert_eq!(ctx.created() <= &Local::now(), true);
    /// ```
    pub fn created(&self) -> &DateTime<Local> {
        self.req.created_ref()
    }

    /// Request's method
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let req = Request::default();
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://example.com")));
    ///
    /// assert_eq!(ctx.uri(), "http://example.com/");
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("https://example.com")));
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://user:pass@example.com")));
    ///
    /// assert_eq!(ctx.authority(), Some(&Authority::from_static("user:pass@example.com")));
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://user:pass@example.com")));
    ///
    /// assert_eq!(ctx.domain(), "example.com");
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://user:pass@git.example.com")));
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://user:pass@git.example.com")));
    ///
    /// assert_eq!(ctx.host(), "git.example.com");
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://example.com:3000")));
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://example.com/repo/fibra")));
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://example.com/?foo=bar")));
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://example.com/?foo=bar&key=%E4%BD%A0%E5%A5%BD")));
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
    /// let ctx = Context::from(Request::default().uri(Uri::from_static("http://user:pass@git.example.com/repo/fibra?foo=bar&key=%E4%BD%A0%E5%A5%BD")));
    ///
    /// assert_eq!(ctx.href(), "http://user:pass@git.example.com/repo/fibra?foo=bar&key=%E4%BD%A0%E5%A5%BD".to_string());
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

    pub fn params(&self) -> &IndexMap<String, String> {
        &self.params
    }

    pub fn param(&self, key: &str) -> &str {
        self.params.get(key).map(|v| v.as_str()).unwrap_or("")
    }
}

impl Context {
    pub async fn read_all(&mut self) -> BufList {
        let mut list = BufList::new();
        while let Some(bytes) = self.read_chunk().await {
            list.push_chunk(bytes);
        }
        list
    }

    pub async fn read_chunk(&mut self) -> Option<Bytes> {
        use hyper::body::HttpBody;
        self.req.body_mut().data().await.and_then(|r| r.ok())
    }
}

impl Context {
    #[inline]
    pub fn push(&mut self, cur: *const dyn Handler) {
        self.routing.push((cur, 0));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.routing.pop();
    }

    pub async fn next(mut self) -> FibraResult<Response> {
        // while let Some((cur, idx)) = self.routing.last_mut() {
        //     let top = unsafe { &**cur };
        //     let cld = match top.select(*idx) {
        //         Some(obj) => obj,
        //         None => {
        //             self.pop();
        //             continue;
        //         }
        //     };
        // 
        //     *idx += 1;
        // 
        //     return cld.handle(self).await;
        // }
        // 
        // Err(FibraError::PathNotFound("todo".into())) // todo

        Ok("abc".into())
    }

    pub fn reset(self) {
        todo!()
    }

    /// Reject current request with FORBIDDEN by default
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     assert_eq!(Context::from(Request::default()).reject(None)?.status_ref(), &Status::FORBIDDEN);
    ///     assert_eq!(Context::from(Request::default()).reject(Some(Status::BAD_REQUEST))?, &Status::BAD_REQUEST);
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
    ///     todo!()
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
    ///     assert_eq!(Context::from(Request::default()).redirect("http://example.com", None)?.status_ref(), &Status::FORBIDDEN);
    ///     assert_eq!(Context::from(Request::default()).redirect("http://example.com", None)?.header_ref(header::LOCATION), &HeaderValue::from_static("http://example.com"));
    ///     assert_eq!(Context::from(Request::default()).redirect("http://example.com", Some(Status::MOVED_PERMANENTLY))?.status_ref(), &Status::BAD_REQUEST);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn redirect(self, to: impl IntoUri, status: Option<Status>) -> FibraResult<Response> {
        let location = HeaderValue::try_from(to.into_uri().to_string())?;
        let redirect = status.unwrap_or(Status::FOUND);
        Ok(Response::default().status(redirect).header(header::LOCATION, location))
    }
}

/// Construct from client request
impl From<(Arc<Fibra>, Arc<Connection>, Request)> for Context {
    fn from((app, conn, req): (Arc<Fibra>, Arc<Connection>, Request)) -> Self {
        // todo push root to routing
        let served = conn.count_add(1);
        let queries = form_urlencoded::parse(req.query().as_bytes()).into_owned().collect();
        Self { app, conn, served, req, params: IndexMap::new(), queries, routing: vec![] }
    }
}

/// For mock use only
impl From<Request> for Context {
    fn from(req: Request) -> Self {
        (Arc::new(Fibra::default()), Arc::new(Connection::default()), req).into()
    }
}
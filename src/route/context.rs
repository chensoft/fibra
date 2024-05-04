//! Request Context
use crate::route::*;
use crate::types::*;

/// Context which holds the connection and request
pub struct Context {
    /// The root app instance
    app: Arc<Router>,

    /// Current connection ref
    conn: Arc<Connection>,

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
    pub fn app(&self) -> &Router {
        &self.app
    }

    /// Current connection, multiple requests may belong to a single connection
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Current connection's unique id
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
    ///
    /// assert_eq!(ctx.connid() > 0, true);
    /// ```
    pub fn connid(&self) -> u128 {
        *self.conn.id_ref()
    }

    /// Current connection's established time
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let old = Local::now();
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
    ///
    /// assert_eq!(ctx.established() >= &old, true);
    /// assert_eq!(ctx.established() <= &Local::now(), true);
    /// ```
    pub fn established(&self) -> &DateTime<Local> {
        self.conn.created_ref()
    }

    /// The number of requests served by a connection
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default().count(5));
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
    ///
    /// assert_eq!(ctx.served(), 5);
    /// ```
    pub fn served(&self) -> usize {
        self.conn.count_ref().load(atomic::Ordering::Relaxed)
    }

    /// The endpoint on the local machine for the connection
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let app = Arc::new(Router::default());
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
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let app = Arc::new(Router::default());
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
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
    ///
    /// assert_eq!(ctx.reqid() > 0, true);
    /// assert_ne!(ctx.reqid(), ctx.connid());
    /// ```
    pub fn reqid(&self) -> u128 {
        *self.req.id_ref()
    }

    /// Request's created time
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let old = Local::now();
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Method};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().method(Method::PUT);
    /// let ctx = Context::from((app, con, req));
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
        self.method() == &Method::GET
    }

    /// Check method
    pub fn is_post(&self) -> bool {
        self.method() == &Method::POST
    }

    /// Check method
    pub fn is_put(&self) -> bool {
        self.method() == &Method::PUT
    }

    /// Check method
    pub fn is_delete(&self) -> bool {
        self.method() == &Method::DELETE
    }

    /// Check method
    pub fn is_patch(&self) -> bool {
        self.method() == &Method::PATCH
    }

    /// Request's uri
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://example.com"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri, Scheme};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("https://example.com"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri, Authority};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://user:pass@example.com"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://user:pass@example.com"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://user:pass@git.example.com"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://user:pass@git.example.com"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://example.com:3000"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://example.com/repo/bolt"));
    /// let ctx = Context::from((app, con, req));
    ///
    /// assert_eq!(ctx.path(), "/repo/bolt");
    /// ```
    pub fn path(&self) -> &str {
        self.req.path()
    }

    /// Request's query value
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://example.com/?foo=bar"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://example.com/?foo=bar&key=%E4%BD%A0%E5%A5%BD"));
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Uri};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().uri(Uri::from_static("http://user:pass@git.example.com/repo/bolt?foo=bar&key=%E4%BD%A0%E5%A5%BD"));
    /// let ctx = Context::from((app, con, req));
    ///
    /// assert_eq!(ctx.href(), "http://user:pass@git.example.com/repo/bolt?foo=bar&key=%E4%BD%A0%E5%A5%BD".to_string());
    /// ```
    pub fn href(&self) -> String {
        self.req.href()
    }

    /// Request's version
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request, Version};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().version(Version::HTTP_10);
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().header("content-type", "application/json").header("cache-control", "no-cache");
    /// let ctx = Context::from((app, con, req));
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
    /// use chrono::Local;
    /// use std::sync::Arc;
    /// use bolt::{Context, Router, Connection, Request};
    ///
    /// let app = Arc::new(Router::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default().header("content-type", "application/json").header("cache-control", "no-cache");
    /// let ctx = Context::from((app, con, req));
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
        // todo body length limit
        use hyper::body::HttpBody;
        self.req.body_mut().data().await.map(|r| r.ok()).flatten()
    }

    pub fn decode(&mut self) {
        todo!()
    }

    pub async fn save(&mut self, _path: &str) {
        todo!()
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

    pub async fn next(mut self) -> BoltResult<Response> {
        while let Some((cur, idx)) = self.routing.last_mut() {
            let top = unsafe { &**cur };
            let cld = match top.child(*idx) {
                Some(obj) => obj,
                None => {
                    self.pop();
                    continue;
                }
            };

            *idx += 1;

            return cld.handle(self).await;
        }

        Ok(Response::default())
    }

    pub async fn reject(self, status: Option<Status>) -> BoltResult<Response> {
        Ok(Response::default().status(status.unwrap_or(Status::FORBIDDEN)))
    }

    pub async fn rewrite(mut self, to: &'static str, _body: Vec<u8>) -> BoltResult<Response> {
        // todo no body
        self.req = self.req.uri(Uri::from_static(to)); // todo right?
        // self.req.body_mut() = ;

        let app = self.app;
        let ctx = Context::from((app.clone(), self.conn, self.req));

        app.handle(ctx).await
    }

    pub async fn redirect(self, to: Uri, status: Option<Status>) -> BoltResult<Response> {
        Ok(Response::default()
            .status(status.unwrap_or(Status::TEMPORARY_REDIRECT))
            .header(header::LOCATION, HeaderValue::from_str(to.to_string().as_str())?))
    }
}

impl From<(Arc<Router>, Arc<Connection>, Request)> for Context {
    fn from((app, conn, req): (Arc<Router>, Arc<Connection>, Request)) -> Self {
        let queries = form_urlencoded::parse(req.query().as_bytes()).into_owned().collect();
        Self { app, conn, req, params: IndexMap::new(), queries, routing: vec![] }
    }
}

// todo mock context
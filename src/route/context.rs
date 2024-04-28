//! Request Context
use crate::fibra::*;
use crate::route::*;
use crate::types::*;

/// Context which hold the connection and request
pub struct Context {
    /// The root app instance
    app: Arc<Fibra>,

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
    pub fn app(&self) -> &Fibra {
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
    /// use fibra::{Fibra, Context, Connection, Request};
    ///
    /// let app = Arc::new(Fibra::default());
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
    /// use fibra::{Fibra, Context, Connection, Request};
    ///
    /// let app = Arc::new(Fibra::default());
    /// let con = Arc::new(Connection::default());
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
    ///
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
    /// use fibra::{Fibra, Context, Connection, Request};
    ///
    /// let app = Arc::new(Fibra::default());
    /// let con = Arc::new(Connection::default().count(5));
    /// let req = Request::default();
    /// let ctx = Context::from((app, con, req));
    ///
    /// assert_eq!(ctx.served(), 5);
    /// ```
    pub fn served(&self) -> u64 {
        *self.conn.count_ref()
    }

    /// The local address that is connected
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    /// use fibra::{Fibra, Context, Connection, Request};
    ///
    /// let app = Arc::new(Fibra::default());
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
    /// use fibra::{Fibra, Context, Connection, Request};
    ///
    /// let app = Arc::new(Fibra::default());
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
    pub fn req(&self) -> &Request {
        &self.req
    }

    pub fn reqid(&self) -> u128 {
        *self.req.id_ref()
    }

    pub fn created(&self) -> &DateTime<Local> {
        self.req.created_ref()
    }

    pub fn method(&self) -> &Method {
        self.req.method_ref()
    }

    pub fn is_get(&self) -> bool {
        self.method() == &Method::GET
    }

    pub fn is_post(&self) -> bool {
        self.method() == &Method::POST
    }

    // todo more methods

    pub fn uri(&self) -> &Uri {
        self.req.uri_ref()
    }

    pub fn scheme(&self) -> &Scheme {
        self.req.scheme()
    }

    pub fn is_secure(&self) -> bool {
        self.req.scheme() == &Scheme::HTTPS
    }

    pub fn authority(&self) -> Option<&Authority> {
        self.req.authority()
    }

    pub fn domain(&self) -> &str {
        self.req.domain()
    }

    pub fn subdomain(&self) -> &str {
        self.req.subdomain()
    }

    pub fn host(&self) -> &str {
        self.req.host()
    }

    pub fn port(&self) -> u16 {
        self.req.port()
    }

    pub fn path(&self) -> &str {
        self.req.path()
    }

    // /// Get a query value
    // ///
    // /// ```
    // /// use fibra::{Request, Uri};
    // ///
    // /// assert_eq!(Request::default().queries("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).queries("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/")).queries("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog")).queries("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?")).queries("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c")).queries("nonce"), "1a2b3c");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c&signature=abcde")).queries("nonce"), "1a2b3c");
    // /// ```
    pub fn query(&self, key: &str) -> &str {
        self.queries.get(key).map(|v| v.as_str()).unwrap_or("")
    }

    pub fn queries(&self) -> &IndexMap<String, String> {
        &self.queries
    }

    pub fn href(&self) -> String {
        self.req.href()
    }

    pub fn version(&self) -> &Version {
        self.req.version_ref()
    }

    pub fn is_http10(&self) -> bool {
        self.version() == &Version::HTTP_10
    }

    pub fn is_http11(&self) -> bool {
        self.version() == &Version::HTTP_11
    }

    pub fn is_http2(&self) -> bool {
        self.version() == &Version::HTTP_2
    }

    pub fn is_http3(&self) -> bool {
        self.version() == &Version::HTTP_3
    }

    pub fn headers(&self) -> &HeaderMap {
        self.req.headers_ref()
    }

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

    pub async fn next(mut self) -> FibraResult<Response> {
        while let Some((cur, idx)) = self.routing.last_mut() {
            let top = unsafe { &**cur };
            let cld = match top.nested(*idx) {
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

    pub async fn reject(self, status: Option<Status>) -> FibraResult<Response> {
        Ok(Response::default().status(status.unwrap_or(Status::FORBIDDEN)))
    }

    pub async fn rewrite(mut self, to: &'static str, _body: Vec<u8>) -> FibraResult<Response> {
        // todo no body
        self.req = self.req.uri(Uri::from_static(to)); // todo right?
        // self.req.body_mut() = ;

        let app = self.app;
        let ctx = Context::from((app.clone(), self.conn, self.req));

        app.handle(ctx).await
    }

    pub async fn redirect(self, to: Uri, status: Option<Status>) -> FibraResult<Response> {
        Ok(Response::default()
            .status(status.unwrap_or(Status::TEMPORARY_REDIRECT))
            .header(header::LOCATION, HeaderValue::from_str(to.to_string().as_str())?))
    }
}

impl From<(Arc<Fibra>, Arc<Connection>, Request)> for Context {
    fn from((app, conn, req): (Arc<Fibra>, Arc<Connection>, Request)) -> Self {
        let queries = form_urlencoded::parse(req.query().as_bytes()).into_owned().collect();
        Self { app, conn, req, params: IndexMap::new(), queries, routing: vec![] }
    }
}
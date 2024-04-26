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
    param: IndexMap<String, String>,

    /// The query string of the Uri
    query: IndexMap<String, String>,

    /// Internal routing stack
    route: Vec<(*const dyn Handler, usize)>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn app(&self) -> &Fibra {
        &self.app
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    pub fn connid(&self) -> u128 {
        *self.conn.id_ref()
    }

    pub fn established(&self) -> &DateTime<Local> {
        self.conn.created_ref()
    }

    pub fn served(&self) -> u64 {
        *self.conn.count_ref()
    }

    pub fn local(&self) -> &SocketAddr {
        self.conn.sockaddr_ref()
    }

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
    // /// assert_eq!(Request::default().query("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).query("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/")).query("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog")).query("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?")).query("nonce"), "");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c")).query("nonce"), "1a2b3c");
    // /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c&signature=abcde")).query("nonce"), "1a2b3c");
    // /// ```
    // pub fn query(&self, key: &str) -> &str {
    //     // todo performance
    //     self.query.get(key).map(|v| v.as_str()).unwrap_or("")
    // }
    pub fn query(&self, key: &str) -> &str {
        self.query.get(key).map(|v| v.as_str()).unwrap_or("")
    }

    pub fn queries(&self) -> &IndexMap<String, String> {
        &self.query
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

    pub fn header(&self, key: impl header::AsHeaderName) -> Option<&HeaderValue> {
        self.req.header_ref(key)
    }

    pub fn params(&self) -> &IndexMap<String, String> {
        &self.param
    }

    pub fn param(&self, _key: &str) -> &str {
        todo!()
    }
}

impl Context {
    // todo check cqueues
    pub async fn read(&mut self) {
        todo!()
    }

    pub async fn read_line(&mut self) {
        todo!()
    }

    pub async fn read_until(&mut self) {
        todo!()
    }

    pub async fn save(&mut self, _path: &str) {
        todo!()
    }

    // todo peek
}

impl Context {
    #[inline]
    pub fn push(&mut self, cur: *const dyn Handler) {
        self.route.push((cur, 0));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.route.pop();
    }

    pub async fn next(mut self) -> FibraResult<Response> {
        while let Some((cur, idx)) = self.route.last_mut() {
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
        let query = form_urlencoded::parse(req.query().as_bytes()).into_owned().collect();
        Self { app, conn, req, param: IndexMap::new(), query, route: vec![] }
    }
}
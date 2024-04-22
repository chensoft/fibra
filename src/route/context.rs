use crate::fibra::*;
use crate::route::*;
use crate::types::*;

// todo default generic type: pub struct HeaderMap<T = HeaderValue>
pub struct Context {
    app: Arc<Fibra>, // todo set config in fibra, read from this
    req: Request,

    param: IndexMap<String, String>, // todo radix map
    stack: Vec<(*const dyn Handler, usize)>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(app: Arc<Fibra>, req: Request) -> Self {
        Self { app, req, param: IndexMap::new(), stack: vec![] }
    }

    pub fn app(&self) -> &Fibra {
        &self.app
    }

    pub fn req(&self) -> &Request {
        &self.req
    }

    pub fn id(&self) -> &u128 {
        self.req.id_ref()
    }

    pub fn created(&self) -> &DateTime<Local> {
        self.req.created_ref()
    }

    pub fn sockaddr(&self) -> &SocketAddr {
        self.req.sockaddr_ref()
    }

    pub fn peeraddr(&self) -> &SocketAddr {
        self.req.peeraddr_ref()
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
        self.req.scheme() == &Scheme::HTTPS // todo wss
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

    pub fn queries(&self) -> &str {
        self.req.queries()
    }

    pub fn query(&self, _key: &str) -> &str {
        // todo use self.form
        todo!()
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
        self.stack.push((cur, 0));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub async fn next(mut self) -> FibraResult<Response> {
        while let Some((cur, idx)) = self.stack.last_mut() {
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

    pub async fn rewrite(mut self, to: &'static str, body: Vec<u8>) -> FibraResult<Response> {
        // todo no body
        self.req = self.req.uri(Uri::from_static(to)); // todo right?
        // self.req.body_mut() = ;

        let app = self.app;
        let ctx = Context::new(app.clone(), self.req);

        app.handle(ctx).await
    }

    pub async fn redirect(self, to: Uri, status: Option<Status>) -> FibraResult<Response> {
        Ok(Response::default()
            .status(status.unwrap_or(Status::TEMPORARY_REDIRECT))
            .header(header::LOCATION, HeaderValue::from_str(to.to_string().as_str())?))
    }
}
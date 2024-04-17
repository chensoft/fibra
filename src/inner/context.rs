use crate::fibra::*;
use crate::types::*;
use crate::inner::*;

// todo default generic type: pub struct HeaderMap<T = HeaderValue>
pub struct Context {
    uniq: u64, // todo request id how to uniq? atom
    time: DateTime<Local>,
    root: Arc<Fibra>, // todo set config in fibra, read from this
    sock: SocketAddr,
    peer: SocketAddr,
    head: Parts,
    capt: IndexMap<String, String>, // todo radix map
    form: IndexMap<String, String>, // todo query map
    body: Body,
    lifo: Vec<(*const dyn Handler, usize)>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(root: Arc<Fibra>, sock: SocketAddr, peer: SocketAddr, req: Request<Body>) -> Self {
        let (head, body) = req.into_parts();
        Self { uniq: 0, time: Local::now(), root, sock, peer, head, capt: IndexMap::new(), form: IndexMap::new(), body, lifo: vec![] } // todo
    }

    pub fn identifier(&self) -> u64 {
        self.uniq
    }

    pub fn beginning(&self) -> &DateTime<Local> {
        &self.time
    }

    pub fn router(&self) -> &Fibra {
        &self.root
    }

    pub fn server(&self) -> &SocketAddr {
        &self.sock
    }

    pub fn client(&self) -> &SocketAddr {
        &self.peer
    }

    // todo realip and port use addon

    // todo tls info, ver, sni

    pub fn method(&self) -> &Method {
        &self.head.method
    }

    pub fn scheme(&self) -> &Scheme {
        self.head.uri.scheme().unwrap_or_else(|| unreachable!()) // todo ws, wss
    }

    pub fn secure(&self) -> bool {
        self.scheme() == &Scheme::HTTPS || self.scheme().as_str() == "wss"
    }

    pub fn authority(&self) -> Option<&Authority> {
        self.head.uri.authority()
    }

    pub fn subdomain(&self) -> &str {
        todo!()
    }

    pub fn host(&self) -> &str {
        self.head.uri.host().unwrap_or("")
    }

    pub fn port(&self) -> u16 {
        match self.head.uri.port_u16() {
            Some(port) => port,
            None => match self.scheme() {
                scheme if scheme == &Scheme::HTTP || scheme.as_str() == "ws" => 80,
                scheme if scheme == &Scheme::HTTPS || scheme.as_str() == "wss" => 443,
                _ => unreachable!()
            }
        }
    }

    pub fn path(&self) -> &str {
        &self.head.uri.path()
    }

    pub fn queries(&self) -> &str {
        self.head.uri.query().unwrap_or("")
    }

    pub fn query(&self, _key: &str) -> &str {
        // todo use self.form
        todo!()
    }

    pub fn params(&self) -> &IndexMap<String, String> {
        &self.capt
    }

    pub fn param(&self, _key: &str) -> &str {
        todo!()
    }

    pub fn href(&self) -> String {
        self.head.uri.to_string()
    }

    pub fn version(&self) -> &Version {
        &self.head.version
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.head.headers
    }

    pub fn header(&self, key: &HeaderName) -> Option<&HeaderValue> {
        self.head.headers.get(key.as_str())
    }

    pub fn cookies(&self) {
        todo!()
    }

    pub fn cookie(&self, _key: &str) -> Vec<&str> {
        todo!()
    }
}

impl Context {
    pub async fn read(&mut self) {
        todo!()
    }

    pub async fn reject(&mut self, status: Option<StatusCode>) -> FibraResult<Response<Body>> {
        Ok(status.unwrap_or(StatusCode::FORBIDDEN).into_response())
    }

    pub async fn rewrite(mut self, to: &'static str, body: Vec<u8>) -> FibraResult<Response<Body>> {
        // todo no body
        self.head.uri = Uri::from_static(to); // todo right?

        let app = self.root;
        let ctx = Context::new(app.clone(), self.sock, self.peer, Request::from_parts(self.head, Body::from(body)));

        app.handle(ctx).await
    }

    pub async fn redirect(&mut self, to: Uri, status: Option<StatusCode>) -> FibraResult<Response<Body>> {
        Ok(Response::builder()
            .status(status.unwrap_or(StatusCode::TEMPORARY_REDIRECT))
            .header(header::LOCATION, header::HeaderValue::from_str(to.to_string().as_str())?)
            .body(Body::empty())?)
    }
}

impl Context {
    #[inline]
    pub fn push(&mut self, cur: *const dyn Handler) {
        self.lifo.push((cur, 0));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.lifo.pop();
    }

    pub async fn next(mut self) -> FibraResult<Response<Body>> {
        while let Some((cur, idx)) = self.lifo.last_mut() {
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
}
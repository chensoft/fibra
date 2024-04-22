use crate::types::*;

pub struct Request {
    id: u128,
    created: DateTime<Local>,

    sockaddr: SocketAddr,
    peeraddr: SocketAddr,

    method: Method,
    uri: Uri,
    query: IndexMap<String, String>,
    version: Version,
    headers: HeaderMap,
    body: Body,
}

impl Request {
    pub fn new(sock: impl Into<SocketAddr>, peer: impl Into<SocketAddr>, from: hyper::Request<Body>) -> Self {
        let (head, body) = from.into_parts();
        Self {
            id: Ulid::new().0,
            created: Local::now(),
            sockaddr: sock.into(),
            peeraddr: peer.into(),
            method: head.method,
            uri: head.uri,
            query: IndexMap::new(),
            version: head.version,
            headers: head.headers,
            body,
        }
    }

    pub fn id_ref(&self) -> &u128 {
        &self.id
    }

    pub fn id_mut(&mut self) -> &mut u128 {
        &mut self.id
    }

    pub fn id(mut self, val: u128) -> Self {
        self.id = val;
        self
    }

    pub fn created_ref(&self) -> &DateTime<Local> {
        &self.created
    }

    pub fn created_mut(&mut self) -> &mut DateTime<Local> {
        &mut self.created
    }

    pub fn created(mut self, val: DateTime<Local>) -> Self {
        self.created = val;
        self
    }

    pub fn sockaddr_ref(&self) -> &SocketAddr {
        &self.sockaddr
    }

    pub fn sockaddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.sockaddr
    }

    pub fn sockaddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.sockaddr = val.into();
        self
    }

    pub fn peeraddr_ref(&self) -> &SocketAddr {
        &self.peeraddr
    }

    pub fn peeraddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.peeraddr
    }

    pub fn peeraddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.peeraddr = val.into();
        self
    }

    pub fn method_ref(&self) -> &Method {
        &self.method
    }

    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    pub fn method(mut self, val: impl Into<Method>) -> Self {
        self.method = val.into();
        self
    }

    pub fn uri_ref(&self) -> &Uri {
        &self.uri
    }

    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.uri
    }

    pub fn uri(mut self, val: impl Into<Uri>) -> Self {
        self.uri = val.into();
        self.query.clear();
        self
    }

    pub fn version_ref(&self) -> &Version {
        &self.version
    }

    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    pub fn version(mut self, val: impl Into<Version>) -> Self {
        self.version = val.into();
        self
    }

    pub fn headers_ref(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub fn headers(mut self, val: HeaderMap) -> Self {
        self.headers = val;
        self
    }

    pub fn header_ref(&self, key: impl header::AsHeaderName) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    pub fn header_mut(&mut self, key: impl header::AsHeaderName) -> Option<&mut HeaderValue> {
        self.headers.get_mut(key)
    }

    pub fn header(mut self, key: impl header::IntoHeaderName, val: HeaderValue) -> Self {
        self.headers.insert(key, val);
        self
    }

    pub fn body_ref(&self) -> &Body {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn body(mut self, val: impl Into<Body>) -> Self {
        self.body = val.into();
        self
    }
}

impl Request {
    pub fn scheme(&self) -> &Scheme {
        todo!() // check tls socket
    }

    pub fn authority(&self) -> Option<&Authority> {
        self.uri.authority()
    }

    pub fn subdomain(&self) -> &str {
        todo!()
    }

    pub fn host(&self) -> &str {
        self.uri.host().unwrap_or("")
    }

    pub fn port(&self) -> u16 {
        match self.uri.port_u16() {
            Some(port) => port,
            None => match self.scheme() {
                Scheme::HTTP => 80,
                Scheme::HTTPS => 443
            }
        }
    }

    pub fn path(&self) -> &str {
        &self.uri.path()
    }

    pub fn queries(&self) -> &str {
        self.uri.query().unwrap_or("")
    }

    pub fn query(&self, _key: &str) -> &str {
        todo!()
    }

    pub fn href(&self) -> String {
        self.uri.to_string()
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new(([0, 0, 0, 0], 0), ([0, 0, 0, 0], 0), hyper::Request::default())
    }
}
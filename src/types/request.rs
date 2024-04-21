use crate::types::*;
use hyper::http::request::Parts;

pub struct Request {
    uniq: u64, // todo request id how to uniq? atom
    time: DateTime<Local>,
    sock: SocketAddr,
    peer: SocketAddr,
    head: Parts,
    body: Body,
}

// todo define trait and impl for context
impl Request {
    pub fn new(sock: SocketAddr, peer: SocketAddr, from: hyper::Request<Body>) -> Self {
        let (head, body) = from.into_parts();
        Self { uniq: 0, time: Local::now(), sock, peer, head, body }
    }

    pub fn identifier(&self) -> u64 {
        self.uniq
    }

    pub fn beginning(&self) -> &DateTime<Local> {
        &self.time
    }

    pub fn server(&self) -> &SocketAddr {
        &self.sock
    }

    pub fn client(&self) -> &SocketAddr {
        &self.peer
    }

    pub fn parts_ref(&self) -> &Parts {
        &self.head
    }

    pub fn parts_mut(&mut self) -> &mut Parts {
        &mut self.head
    }

    pub fn body_ref(&self) -> &Body {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    // todo realip and port use addon

    // todo tls info, ver, sni

    pub fn method(&self) -> &Method {
        &self.head.method
    }

    pub fn uri(&self) -> &Uri {
        &self.head.uri
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

impl Default for Request {
    fn default() -> Self {
        todo!()
    }
}
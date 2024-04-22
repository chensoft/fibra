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

impl Request {
    pub fn new(sock: SocketAddr, peer: SocketAddr, from: hyper::Request<Body>) -> Self {
        let (head, body) = from.into_parts();
        Self { uniq: 0, time: Local::now(), sock, peer, head, body }
    }

    pub fn id_ref(&self) -> u64 {
        self.uniq
    }

    pub fn id_mut(&mut self) -> &mut u64 {
        &mut self.uniq
    }

    pub fn id(mut self) -> Self {
        todo!()
    }

    pub fn time_ref(&self) -> &DateTime<Local> {
        &self.time
    }

    pub fn time_mut(&mut self) -> &mut DateTime<Local> {
        &mut self.time
    }

    pub fn time(mut self) -> Self {
        todo!()
    }

    pub fn server_ref(&self) -> &SocketAddr {
        &self.sock
    }

    pub fn server_mut(&mut self) -> &mut SocketAddr {
        &mut self.sock
    }

    pub fn server(mut self) -> Self {
        todo!()
    }

    pub fn client_ref(&self) -> &SocketAddr {
        &self.peer
    }

    pub fn client_mut(&mut self) -> &mut SocketAddr {
        &mut self.peer
    }

    pub fn client(mut self) -> Self {
        todo!()
    }

    pub fn parts_ref(&self) -> &Parts {
        &self.head
    }

    pub fn parts_mut(&mut self) -> &mut Parts {
        &mut self.head
    }

    pub fn parts(mut self) -> Self {
        todo!()
    }

    pub fn body_ref(&self) -> &Body {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn body(mut self) -> Self {
        todo!()
    }

    // todo realip and port use addon

    // todo tls info, ver, sni

    pub fn method_ref(&self) -> &Method {
        &self.head.method
    }
    pub fn method_mut(&self) -> &Method {
        &self.head.method
    }
    pub fn method(&self) -> &Method {
        &self.head.method
    }

    pub fn uri_ref(&self) -> &Uri {
        &self.head.uri
    }
    pub fn uri_mut(&self) -> &Uri {
        &self.head.uri
    }
    pub fn uri(&self) -> &Uri {
        &self.head.uri
    }

    pub fn version_ref(&self) -> &Version {
        &self.head.version
    }
    pub fn version_mut(&self) -> &Version {
        &self.head.version
    }
    pub fn version(&self) -> &Version {
        &self.head.version
    }

    pub fn headers_ref(&self) -> &HeaderMap {
        &self.head.headers
    }
    pub fn headers_mut(&self) -> &HeaderMap {
        &self.head.headers
    }
    pub fn headers(&self) -> &HeaderMap {
        &self.head.headers
    }

    pub fn header_ref(&self, key: &HeaderName) -> Option<&HeaderValue> {
        self.head.headers.get(key.as_str())
    }

    pub fn header_mut(&self, key: &HeaderName) -> Option<&HeaderValue> {
        self.head.headers.get(key.as_str())
    }

    pub fn header(&self, key: &HeaderName) -> Option<&HeaderValue> {
        self.head.headers.get(key.as_str())
    }
}

impl Request {
    pub fn is_get(&self) -> bool {
        self.method_ref() == Method::GET
    }

    pub fn is_post(&self) -> bool {
        self.method_ref() == Method::POST
    }

    // todo more methods

    pub fn scheme(&self) -> &Scheme {
        self.head.uri.scheme().unwrap_or_else(|| unreachable!()) // todo ws, wss
    }

    pub fn is_secure(&self) -> bool {
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

    // todo cookie
}

impl Default for Request {
    fn default() -> Self {
        todo!()
    }
}
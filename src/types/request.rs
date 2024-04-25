//! HTTP Request
use crate::types::*;

/// A request represents a single request-response cycle. Multiple requests may exist on a
/// single connection if the client is using HTTP/1.1's keep-alive feature or HTTP/2
pub struct Request {
    /// The unique identifier of this request
    id: u128,

    /// The time the request was created
    created: DateTime<Local>,

    /// The local address that is connected
    sockaddr: SocketAddr,

    /// The remote address that the connection comes from or connects to
    peeraddr: SocketAddr,

    /// GET, POST, ...
    method: Method,

    /// The URI component of this request
    ///
    /// This example comes from hyper:
    ///
    /// ```notrust
    /// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
    /// |-|   |-------------------------------||--------| |-------------------| |-----|
    ///  |                  |                       |               |              |
    /// scheme          authority                 path            query         fragment
    /// ```
    uri: Uri,

    /// The query string of the Uri
    query: IndexMap<String, String>,

    /// 1.0, 1.1, 2, ...
    version: Version,

    /// The headers of this request
    headers: HeaderMap,

    /// The stream body of this request
    body: Body,
}

impl Request {
    /// Get the unique identifier of this request
    ///
    /// ```
    /// use fibra::{Request};
    ///
    /// assert_eq!(*Request::default().id_ref() > 0, true);
    /// ```
    pub fn id_ref(&self) -> &u128 {
        &self.id
    }

    /// Get/Set the unique identifier of this request
    ///
    /// ```
    /// use fibra::{Request};
    ///
    /// let mut req = Request::default();
    /// *req.id_mut() = 12345;
    ///
    /// assert_eq!(req.id_ref(), &12345);
    /// ```
    pub fn id_mut(&mut self) -> &mut u128 {
        &mut self.id
    }

    /// Set the unique identifier of this request
    ///
    /// ```
    /// use fibra::{Request};
    ///
    /// assert_eq!(Request::default().id(12345).id_ref(), &12345);
    /// ```
    pub fn id(mut self, val: u128) -> Self {
        self.id = val;
        self
    }

    /// Get the created time of this request
    ///
    /// ```
    /// use chrono::Local;
    /// use fibra::{Request};
    ///
    /// assert_eq!(Request::default().created_ref() <= &Local::now(), true);
    /// ```
    pub fn created_ref(&self) -> &DateTime<Local> {
        &self.created
    }

    /// Get/Set the created time of this request
    ///
    /// ```
    /// use chrono::Local;
    /// use fibra::{Request};
    ///
    /// let now = Local::now();
    /// let mut req = Request::default();
    /// *req.created_mut() = now;
    ///
    /// assert_eq!(req.created_ref(), &now);
    /// ```
    pub fn created_mut(&mut self) -> &mut DateTime<Local> {
        &mut self.created
    }

    /// Set the created time of this request
    ///
    /// ```
    /// use chrono::Local;
    /// use fibra::{Request};
    ///
    /// let now = Local::now();
    /// let req = Request::default().created(now);
    ///
    /// assert_eq!(req.created_ref(), &now);
    /// ```
    pub fn created(mut self, val: DateTime<Local>) -> Self {
        self.created = val;
        self
    }

    /// Get the local address
    ///
    /// ```
    /// use fibra::{Request};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Request::default().sockaddr_ref(), &SocketAddr::from(([0, 0, 0, 0], 0)));
    /// ```
    pub fn sockaddr_ref(&self) -> &SocketAddr {
        &self.sockaddr
    }

    /// Get/Set the local address
    ///
    /// ```
    /// use fibra::{Request};
    /// use std::net::SocketAddr;
    ///
    /// let mut req = Request::default();
    /// *req.sockaddr_mut() = SocketAddr::from(([127, 0, 0, 1], 3000));
    ///
    /// assert_eq!(req.sockaddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn sockaddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.sockaddr
    }

    /// Set the local address
    ///
    /// ```
    /// use fibra::{Request};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Request::default().sockaddr(([127, 0, 0, 1], 3000)).sockaddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn sockaddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.sockaddr = val.into();
        self
    }

    /// Get the remote address
    ///
    /// ```
    /// use fibra::{Request};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Request::default().peeraddr_ref(), &SocketAddr::from(([0, 0, 0, 0], 0)));
    /// ```
    pub fn peeraddr_ref(&self) -> &SocketAddr {
        &self.peeraddr
    }

    /// Get/Set the remote address
    ///
    /// ```
    /// use fibra::{Request};
    /// use std::net::SocketAddr;
    ///
    /// let mut req = Request::default();
    /// *req.peeraddr_mut() = SocketAddr::from(([127, 0, 0, 1], 3000));
    ///
    /// assert_eq!(req.peeraddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn peeraddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.peeraddr
    }

    /// Set the remote address
    ///
    /// ```
    /// use fibra::{Request};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Request::default().peeraddr(([127, 0, 0, 1], 3000)).peeraddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn peeraddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.peeraddr = val.into();
        self
    }

    /// Get the http method
    ///
    /// ```
    /// use fibra::{Request, Method};
    ///
    /// assert_eq!(Request::default().method_ref(), &Method::GET);
    /// ```
    pub fn method_ref(&self) -> &Method {
        &self.method
    }

    /// Get/Set the http method
    ///
    /// ```
    /// use fibra::{Request, Method};
    ///
    /// let mut req = Request::default();
    /// *req.method_mut() = Method::PUT;
    ///
    /// assert_eq!(req.method_ref(), &Method::PUT);
    /// ```
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    /// Set the http method
    ///
    /// ```
    /// use fibra::{Request, Method};
    ///
    /// assert_eq!(Request::default().method(Method::PUT).method_ref(), &Method::PUT);
    /// ```
    pub fn method(mut self, val: impl Into<Method>) -> Self {
        self.method = val.into();
        self
    }

    /// Get the uri
    ///
    /// ```
    /// use fibra::{Request};
    ///
    /// assert_eq!(Request::default().uri_ref().host(), None);
    /// ```
    pub fn uri_ref(&self) -> &Uri {
        &self.uri
    }

    /// Set the uri
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).host(), "chensoft.com");
    /// ```
    pub fn uri(mut self, val: impl Into<Uri>) -> Self {
        self.uri = val.into();
        self.query = form_urlencoded::parse(self.query_raw().as_bytes()).into_owned().collect();
        self
    }

    /// Get the http version
    ///
    /// ```
    /// use fibra::{Request, Version};
    ///
    /// assert_eq!(Request::default().version_ref(), &Version::HTTP_11);
    /// ```
    pub fn version_ref(&self) -> &Version {
        &self.version
    }

    /// Get/Set the http version
    ///
    /// ```
    /// use fibra::{Request, Version};
    ///
    /// let mut req = Request::default();
    /// *req.version_mut() = Version::HTTP_2;
    ///
    /// assert_eq!(req.version_ref(), &Version::HTTP_2);
    /// ```
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    /// Set the http version
    ///
    /// ```
    /// use fibra::{Request, Version};
    ///
    /// assert_eq!(Request::default().version(Version::HTTP_2).version_ref(), &Version::HTTP_2);
    /// ```
    pub fn version(mut self, val: impl Into<Version>) -> Self {
        self.version = val.into();
        self
    }

    /// Get the headers
    ///
    /// ```
    /// use fibra::{Request};
    ///
    /// assert_eq!(Request::default().headers_ref().is_empty(), true);
    /// ```
    pub fn headers_ref(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get/Set the headers
    ///
    /// ```
    /// use fibra::{Request, IntoHeaderValue, header};
    ///
    /// let mut req = Request::default();
    /// req.headers_mut().insert(header::CACHE_CONTROL, "no-cache".into_value());
    ///
    /// assert_eq!(req.headers_ref().get(header::CACHE_CONTROL).map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// ```
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Set the headers
    ///
    /// ```
    /// use fibra::{Request, HeaderMap, IntoHeaderValue, header};
    ///
    /// let mut map = HeaderMap::new();
    /// map.insert(header::CACHE_CONTROL, "no-cache".into_value());
    ///
    /// let mut req = Request::default().headers(map);
    ///
    /// assert_eq!(req.headers_ref().get(header::CACHE_CONTROL).map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// ```
    pub fn headers(mut self, val: HeaderMap) -> Self {
        self.headers = val;
        self
    }

    /// Get a header
    ///
    /// ```
    /// use fibra::{Request, header};
    ///
    /// assert_eq!(Request::default().header_ref(header::ACCEPT_ENCODING).is_none(), true);
    /// ```
    pub fn header_ref(&self, key: impl AsHeaderName) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    /// Get/Set a header
    ///
    /// ```
    /// use fibra::{Request, header};
    ///
    /// assert_eq!(Request::default().header_mut(header::ACCEPT_ENCODING).is_none(), true);
    /// ```
    pub fn header_mut(&mut self, key: impl AsHeaderName) -> Option<&mut HeaderValue> {
        self.headers.get_mut(key)
    }

    /// Set a header
    ///
    /// ```
    /// use fibra::{Request, header};
    ///
    /// let req = Request::default().header(header::ACCEPT_ENCODING, "gzip, deflate");
    ///
    /// assert_eq!(req.header_ref(header::ACCEPT_ENCODING).map(|v| v.as_bytes()), Some("gzip, deflate".as_bytes()));
    /// ```
    pub fn header(mut self, key: impl IntoHeaderName, val: impl IntoHeaderValue) -> Self {
        self.headers.insert(key, val.into_value());
        self
    }

    /// ```
    /// use fibra::{Request};
    ///
    /// Request::default().body_ref();
    /// ```
    pub fn body_ref(&self) -> &Body {
        &self.body
    }

    /// ```
    /// use bytes::Bytes;
    /// use fibra::{Request, FibraResult, body};
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut res = Request::default().body("Hello World!");
    ///     assert_eq!(body::to_bytes(res.body_mut()).await?, Bytes::from("Hello World!"));
    ///     Ok(())
    /// }
    /// ```
    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    ///
    pub fn body(mut self, val: impl Into<Body>) -> Self {
        self.body = val.into();
        self
    }
}

impl Request {
    /// Get the scheme
    ///
    /// ```
    /// use fibra::{Request, Scheme, Uri};
    ///
    /// assert_eq!(Request::default().uri(Uri::from_static("chensoft.com")).scheme(), &Scheme::Unknown);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).scheme(), &Scheme::HTTP);
    /// assert_eq!(Request::default().uri(Uri::from_static("https://chensoft.com")).scheme(), &Scheme::HTTPS);
    /// ```
    pub fn scheme(&self) -> &Scheme {
        // todo check tls socket, scheme is none when self comes from hyper connection
        let scheme = self.uri.scheme();

        if scheme == Some(&hyper::http::uri::Scheme::HTTP) {
            return &Scheme::HTTP;
        }

        if scheme == Some(&hyper::http::uri::Scheme::HTTPS) {
            return &Scheme::HTTPS;
        }

        &Scheme::Unknown
    }

    /// Get the authority
    ///
    /// ```
    /// use fibra::{Request, Uri, Authority};
    ///
    /// assert_eq!(Request::default().uri(Uri::from_static("http://user:pass@chensoft.com")).authority(), Some(&Authority::from_static("user:pass@chensoft.com")));
    /// ```
    pub fn authority(&self) -> Option<&Authority> {
        self.uri.authority()
    }

    /// Get the subdomain of the host
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().subdomain(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).subdomain(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://www.chensoft.com")).subdomain(), "www");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://fibra.api.chensoft.com")).subdomain(), "fibra.api");
    /// assert_eq!(Request::default().uri(Uri::from_static("https://www.google.com.hk")).subdomain(), "www");
    /// ```
    pub fn subdomain(&self) -> &str {
        let host = self.host();
        let domain = match psl::domain(host.as_bytes()) {
            Some(d) => d,
            None => return "",
        };

        match host.len() > domain.as_bytes().len() {
            true => &host[..host.len() - domain.as_bytes().len() - 1],
            false => ""
        }
    }

    /// Get the host
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().host(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).host(), "chensoft.com");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://www.chensoft.com")).host(), "www.chensoft.com");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://fibra.api.chensoft.com")).host(), "fibra.api.chensoft.com");
    /// ```
    pub fn host(&self) -> &str {
        self.uri.host().unwrap_or("")
    }

    /// Get the port
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().port(), 0);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).port(), 80);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com:3000")).port(), 3000);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com:3000/blog")).port(), 3000);
    /// assert_eq!(Request::default().uri(Uri::from_static("https://chensoft.com")).port(), 443);
    /// assert_eq!(Request::default().uri(Uri::from_static("https://www.chensoft.com:8443")).port(), 8443);
    /// ```
    pub fn port(&self) -> u16 {
        match self.uri.port_u16() {
            Some(port) => port,
            None => match self.scheme() {
                Scheme::HTTP => 80,
                Scheme::HTTPS => 443,
                Scheme::Unknown => 0,
            }
        }
    }

    /// Get the path
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog")).path(), "/blog");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog/2024")).path(), "/blog/2024");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com:3000")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com:3000/")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com:3000/blog")).path(), "/blog");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com:3000/blog/2024")).path(), "/blog/2024");
    /// ```
    pub fn path(&self) -> &str {
        self.uri.path()
    }

    /// Get the query string
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().query_raw(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).query_raw(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/")).query_raw(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog")).query_raw(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?")).query_raw(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c")).query_raw(), "nonce=1a2b3c");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c&signature=abcde")).query_raw(), "nonce=1a2b3c&signature=abcde");
    /// ```
    pub fn query_raw(&self) -> &str {
        self.uri.query().unwrap_or("")
    }

    /// Get a query value
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().query("nonce"), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).query("nonce"), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/")).query("nonce"), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog")).query("nonce"), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?")).query("nonce"), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c")).query("nonce"), "1a2b3c");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c&signature=abcde")).query("nonce"), "1a2b3c");
    /// ```
    pub fn query(&self, key: &str) -> &str {
        // todo performance
        self.query.get(key).map(|v| v.as_str()).unwrap_or("")
    }

    /// Get the whole uri
    ///
    /// ```
    /// use fibra::{Request, Uri};
    ///
    /// assert_eq!(Request::default().href(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com")).href(), "http://chensoft.com/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/")).href(), "http://chensoft.com/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog")).href(), "http://chensoft.com/blog");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?")).href(), "http://chensoft.com/blog?");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c")).href(), "http://chensoft.com/blog?nonce=1a2b3c");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://chensoft.com/blog?nonce=1a2b3c&signature=abcde")).href(), "http://chensoft.com/blog?nonce=1a2b3c&signature=abcde");
    /// ```
    pub fn href(&self) -> String {
        self.uri.to_string()
    }
}

/// Default trait
impl Default for Request {
    fn default() -> Self {
        Self::from((([0, 0, 0, 0], 0), ([0, 0, 0, 0], 0), hyper::Request::default()))
    }
}

/// Create a new Request based on hyper's Request
impl<S: Into<SocketAddr>, P: Into<SocketAddr>> From<(S, P, hyper::Request<Body>)> for Request {
    fn from((sock, peer, from): (S, P, hyper::Request<Body>)) -> Self {
        let (head, body) = from.into_parts();
        let query = form_urlencoded::parse(head.uri.query().unwrap_or("").as_bytes()).into_owned().collect();
        Self {
            id: Ulid::new().0,
            created: Local::now(),
            sockaddr: sock.into(),
            peeraddr: peer.into(),
            method: head.method,
            uri: head.uri,
            query,
            version: head.version,
            headers: head.headers,
            body,
        }
    }
}
//! HTTP Request
use crate::types::*;

/// A request represents a single request-response cycle. Multiple requests may exist on a
/// single connection if the client is using HTTP/1x's Keep-Alive feature or HTTP/2
pub struct Request {
    /// The unique identifier of this request
    id: u128,

    /// The time the request was created
    created: DateTime<Local>,

    /// GET, POST, ...
    method: Method,

    /// The URI component of this request
    ///
    /// This example comes from hyper:
    ///
    /// # Examples
    /// 
    /// ```notrust
    /// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
    /// |-|   |-------------------------------||--------| |-------------------| |-----|
    ///  |                  |                       |               |              |
    /// scheme          authority                 path            query         fragment
    /// ```
    uri: Uri,

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
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(*Request::default().id_ref() > 0, true);
    /// ```
    #[inline]
    pub fn id_ref(&self) -> &u128 {
        &self.id
    }

    /// Get/Set the unique identifier of this request
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// let mut req = Request::default();
    /// *req.id_mut() = 12345;
    ///
    /// assert_eq!(req.id_ref(), &12345);
    /// ```
    #[inline]
    pub fn id_mut(&mut self) -> &mut u128 {
        &mut self.id
    }

    /// Set the unique identifier of this request
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().id(12345).id_ref(), &12345);
    /// ```
    #[inline]
    pub fn id(mut self, val: u128) -> Self {
        self.id = val;
        self
    }

    /// Get the created time of this request
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    /// use chrono::Local;
    ///
    /// assert_eq!(Request::default().created_ref() <= &Local::now(), true);
    /// ```
    #[inline]
    pub fn created_ref(&self) -> &DateTime<Local> {
        &self.created
    }

    /// Get/Set the created time of this request
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    /// use chrono::Local;
    ///
    /// let now = Local::now();
    /// let mut req = Request::default();
    /// *req.created_mut() = now;
    ///
    /// assert_eq!(req.created_ref(), &now);
    /// ```
    #[inline]
    pub fn created_mut(&mut self) -> &mut DateTime<Local> {
        &mut self.created
    }

    /// Set the created time of this request
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    /// use chrono::Local;
    ///
    /// let now = Local::now();
    /// let req = Request::default().created(now);
    ///
    /// assert_eq!(req.created_ref(), &now);
    /// ```
    #[inline]
    pub fn created(mut self, val: impl Into<DateTime<Local>>) -> Self {
        self.created = val.into();
        self
    }

    /// Get the http method
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().method_ref(), &Method::GET);
    /// ```
    #[inline]
    pub fn method_ref(&self) -> &Method {
        &self.method
    }

    /// Get/Set the http method
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// let mut req = Request::default();
    /// *req.method_mut() = Method::PUT;
    ///
    /// assert_eq!(req.method_ref(), &Method::PUT);
    /// ```
    #[inline]
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    /// Set the http method
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().method(Method::PUT).method_ref(), &Method::PUT);
    /// ```
    #[inline]
    pub fn method(mut self, val: impl Into<Method>) -> Self {
        self.method = val.into();
        self
    }

    /// Get the uri
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().uri_ref(), "/");
    /// ```
    #[inline]
    pub fn uri_ref(&self) -> &Uri {
        &self.uri
    }

    /// Get/Set the uri
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// let mut req = Request::default();
    /// *req.uri_mut() = Uri::from_static("http://example.com");
    ///
    /// assert_eq!(req.uri_ref(), "http://example.com/");
    /// ```
    #[inline]
    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.uri
    }

    /// Set the uri
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).uri_ref(), "http://example.com/");
    /// ```
    #[inline]
    pub fn uri(mut self, val: impl Into<Uri>) -> Self {
        self.uri = val.into();
        self
    }

    /// Get the http version
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().version_ref(), &Version::HTTP_11);
    /// ```
    #[inline]
    pub fn version_ref(&self) -> &Version {
        &self.version
    }

    /// Get/Set the http version
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// let mut req = Request::default();
    /// *req.version_mut() = Version::HTTP_2;
    ///
    /// assert_eq!(req.version_ref(), &Version::HTTP_2);
    /// ```
    #[inline]
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    /// Set the http version
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().version(Version::HTTP_2).version_ref(), &Version::HTTP_2);
    /// ```
    #[inline]
    pub fn version(mut self, val: impl Into<Version>) -> Self {
        self.version = val.into();
        self
    }

    /// Get the headers
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().headers_ref().is_empty(), true);
    /// ```
    #[inline]
    pub fn headers_ref(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get/Set the headers
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// let mut req = Request::default();
    /// req.headers_mut().insert(header::CACHE_CONTROL, "no-cache".into_value());
    ///
    /// assert_eq!(req.headers_ref().get(header::CACHE_CONTROL).map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Set the headers
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// let mut map = HeaderMap::new();
    /// map.insert(header::CACHE_CONTROL, "no-cache".into_value());
    ///
    /// let mut req = Request::default().headers(map);
    ///
    /// assert_eq!(req.headers_ref().get(header::CACHE_CONTROL).map(|v| v.as_bytes()), Some("no-cache".as_bytes()));
    /// ```
    #[inline]
    pub fn headers(mut self, val: impl Into<HeaderMap>) -> Self {
        self.headers = val.into();
        self
    }

    /// Get a header
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().header_ref(header::ACCEPT_ENCODING).is_none(), true);
    /// ```
    #[inline]
    pub fn header_ref(&self, key: impl AsHeaderName) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    /// Get/Set a header
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().header_mut(header::ACCEPT_ENCODING).is_none(), true);
    /// ```
    #[inline]
    pub fn header_mut(&mut self, key: impl AsHeaderName) -> Option<&mut HeaderValue> {
        self.headers.get_mut(key)
    }

    /// Set a header
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// let req = Request::default().header(header::ACCEPT_ENCODING, "gzip, deflate");
    ///
    /// assert_eq!(req.header_ref(header::ACCEPT_ENCODING).map(|v| v.as_bytes()), Some("gzip, deflate".as_bytes()));
    /// ```
    #[inline]
    pub fn header(mut self, key: impl IntoHeaderName, val: impl IntoHeaderValue) -> Self {
        self.headers.insert(key, val.into_value());
        self
    }

    /// Get the body
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// Request::default().body_ref();
    /// ```
    #[inline]
    pub fn body_ref(&self) -> &Body {
        &self.body
    }

    /// Get the stream body for reading
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    /// use bytes::Bytes;
    ///
    /// #[tokio::main]
    /// async fn main() -> BoltResult<()> {
    ///     let mut res = Request::default().body("Hello World!");
    ///     assert_eq!(body::to_bytes(res.body_mut()).await?, Bytes::from("Hello World!"));
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    /// Set a new body
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    /// use bytes::Bytes;
    ///
    /// #[tokio::main]
    /// async fn main() -> BoltResult<()> {
    ///     assert_eq!(body::to_bytes(Request::default().body("Hello World!").body_mut()).await?, Bytes::from("Hello World!"));
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn body(mut self, val: impl Into<Body>) -> Self {
        self.body = val.into();
        self
    }
}

impl Request {
    /// Get the scheme
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().uri(Uri::from_static("example.com")).scheme(), &Scheme::Unknown);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).scheme(), &Scheme::HTTP);
    /// assert_eq!(Request::default().uri(Uri::from_static("https://example.com")).scheme(), &Scheme::HTTPS);
    /// ```
    #[inline]
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
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().uri(Uri::from_static("http://user:pass@example.com")).authority(), Some(&Authority::from_static("user:pass@example.com")));
    /// ```
    #[inline]
    pub fn authority(&self) -> Option<&Authority> {
        self.uri.authority()
    }

    /// Get the domain from the host
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().domain(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).domain(), "example.com");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://www.example.com")).domain(), "example.com");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://bolt.api.example.com")).domain(), "example.com");
    /// assert_eq!(Request::default().uri(Uri::from_static("https://www.google.com.hk")).domain(), "google.com.hk");
    /// ```
    #[inline]
    pub fn domain(&self) -> &str {
        match psl::domain(self.host().as_bytes()) {
            Some(d) => unsafe { std::str::from_utf8_unchecked(d.as_bytes()) },
            None => return "",
        }
    }

    /// Get the subdomain from the host
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().subdomain(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).subdomain(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://www.example.com")).subdomain(), "www");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://bolt.api.example.com")).subdomain(), "bolt.api");
    /// assert_eq!(Request::default().uri(Uri::from_static("https://www.google.com.hk")).subdomain(), "www");
    /// ```
    #[inline]
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
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().host(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).host(), "example.com");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://www.example.com")).host(), "www.example.com");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://bolt.api.example.com")).host(), "bolt.api.example.com");
    /// ```
    #[inline]
    pub fn host(&self) -> &str {
        self.uri.host().unwrap_or("")
    }

    /// Get the port
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().port(), 0);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).port(), 80);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com:3000")).port(), 3000);
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com:3000/blog")).port(), 3000);
    /// assert_eq!(Request::default().uri(Uri::from_static("https://example.com")).port(), 443);
    /// assert_eq!(Request::default().uri(Uri::from_static("https://www.example.com:8443")).port(), 8443);
    /// ```
    #[inline]
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
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog")).path(), "/blog");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog/2024")).path(), "/blog/2024");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com:3000")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com:3000/")).path(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com:3000/blog")).path(), "/blog");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com:3000/blog/2024")).path(), "/blog/2024");
    /// ```
    #[inline]
    pub fn path(&self) -> &str {
        self.uri.path()
    }

    /// Get the query string
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().query(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).query(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/")).query(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog")).query(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog?")).query(), "");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog?nonce=1a2b3c")).query(), "nonce=1a2b3c");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog?nonce=1a2b3c&signature=abcde")).query(), "nonce=1a2b3c&signature=abcde");
    /// ```
    #[inline]
    pub fn query(&self) -> &str {
        self.uri.query().unwrap_or("")
    }

    /// Get the whole uri
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// assert_eq!(Request::default().href(), "/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com")).href(), "http://example.com/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/")).href(), "http://example.com/");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog")).href(), "http://example.com/blog");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog?")).href(), "http://example.com/blog?");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog?nonce=1a2b3c")).href(), "http://example.com/blog?nonce=1a2b3c");
    /// assert_eq!(Request::default().uri(Uri::from_static("http://example.com/blog?nonce=1a2b3c&signature=abcde")).href(), "http://example.com/blog?nonce=1a2b3c&signature=abcde");
    /// ```
    #[inline]
    pub fn href(&self) -> String {
        self.uri.to_string()
    }
}

impl Default for Request {
    #[inline]
    fn default() -> Self {
        Self::from(hyper::Request::default())
    }
}

/// Create a new Request based on hyper's Request
impl From<hyper::Request<Body>> for Request {
    #[inline]
    fn from(from: hyper::Request<Body>) -> Self {
        let (head, body) = from.into_parts();
        Self {
            id: Ulid::new().0,
            created: Local::now(),
            method: head.method,
            uri: head.uri,
            version: head.version,
            headers: head.headers,
            body,
        }
    }
}
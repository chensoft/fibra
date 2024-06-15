//! HTTP Request
use crate::types::*;

/// A request represents a single request-response cycle. Multiple requests may exist on a
/// single connection if the client is using HTTP/1x's Keep-Alive feature or HTTP/2
pub struct Request {
    /// The time the request was created
    created: SystemTime,

    /// GET, POST, ...
    method: Method,

    /// The URI component of this request
    ///
    /// This example comes from hyper:
    ///
    /// # Examples
    ///
    /// ```notrust
    /// abc://username:password@localip.cc:123/path/data?key=value&key2=value2#fragid1
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
    /// Create a new object
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the created time of this request
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// assert_eq!(Request::new().created_ref() <= &SystemTime::now(), true);
    /// ```
    #[inline]
    pub fn created_ref(&self) -> &SystemTime {
        &self.created
    }

    /// Get/Set the created time of this request
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// let now = SystemTime::now();
    /// let mut req = Request::new();
    /// *req.created_mut() = now;
    ///
    /// assert_eq!(req.created_ref(), &now);
    /// ```
    #[inline]
    pub fn created_mut(&mut self) -> &mut SystemTime {
        &mut self.created
    }

    /// Set the created time of this request
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// let now = SystemTime::now();
    /// let req = Request::new().created(now);
    ///
    /// assert_eq!(req.created_ref(), &now);
    /// ```
    #[inline]
    pub fn created(mut self, val: impl Into<SystemTime>) -> Self {
        self.created = val.into();
        self
    }

    /// Get the http method
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().method_ref(), &Method::GET);
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
    /// use fibra::*;
    ///
    /// let mut req = Request::new();
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().method(Method::PUT).method_ref(), &Method::PUT);
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().uri_ref(), "/");
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
    /// use fibra::*;
    ///
    /// let mut req = Request::new();
    /// *req.uri_mut() = Uri::from_static("http://localip.cc");
    ///
    /// assert_eq!(req.uri_ref(), "http://localip.cc/");
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().uri("http://localip.cc").uri_ref(), "http://localip.cc/");
    /// ```
    #[inline]
    pub fn uri(mut self, val: impl IntoUri) -> Self {
        self.uri = val.into_uri();
        self
    }

    /// Get the http version
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().version_ref(), &Version::HTTP_11);
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
    /// use fibra::*;
    ///
    /// let mut req = Request::new();
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().version(Version::HTTP_2).version_ref(), &Version::HTTP_2);
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().headers_ref().is_empty(), true);
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
    /// use fibra::*;
    ///
    /// let mut req = Request::new();
    /// req.headers_mut().insert(header::CACHE_CONTROL, "no-cache".into_header_value());
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
    /// use fibra::*;
    ///
    /// let mut map = HeaderMap::new();
    /// map.insert(header::CACHE_CONTROL, "no-cache".into_header_value());
    ///
    /// let mut req = Request::new().headers(map);
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().header_ref(header::ACCEPT_ENCODING).is_none(), true);
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().header_mut(header::ACCEPT_ENCODING).is_none(), true);
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
    /// use fibra::*;
    ///
    /// let req = Request::new().header(header::ACCEPT_ENCODING, "gzip, deflate");
    ///
    /// assert_eq!(req.header_ref(header::ACCEPT_ENCODING).map(|v| v.as_bytes()), Some("gzip, deflate".as_bytes()));
    /// ```
    #[inline]
    pub fn header(mut self, key: impl IntoHeaderName, val: impl IntoHeaderValue) -> Self {
        self.headers.insert(key.into_header_name(), val.into_header_value());
        self
    }

    /// Get the body
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// Request::new().body_ref();
    /// ```
    #[inline]
    pub fn body_ref(&self) -> &Body {
        &self.body
    }

    /// Get the stream body for reading
    #[inline]
    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    /// Consume body and turn it into Bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut req = Request::new().body("Hello World!");
    ///     assert_eq!(req.body_all().await?, "Hello World!");
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub async fn body_all(&mut self) -> BufList {
        self.body_mut().read_all().await
    }

    /// Set a new body
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     assert_eq!(Request::new().body("Hello World!").body_all().await?, "Hello World!");
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().uri("localip.cc").scheme(), &Scheme::HTTP);
    /// assert_eq!(Request::new().uri("http://localip.cc").scheme(), &Scheme::HTTP);
    /// assert_eq!(Request::new().uri("https://localip.cc").scheme(), &Scheme::HTTPS);
    /// ```
    #[inline]
    pub fn scheme(&self) -> &Scheme {
        if self.uri.scheme() == Some(&hyper::http::uri::Scheme::HTTPS) {
            return &Scheme::HTTPS;
        }

        &Scheme::HTTP
    }

    /// Get the authority
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().uri("http://user:pass@localip.cc").authority(), Some(&Authority::from_static("user:pass@localip.cc")));
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().domain(), "");
    /// assert_eq!(Request::new().uri("http://localip.cc").domain(), "localip.cc");
    /// assert_eq!(Request::new().uri("http://www.localip.cc").domain(), "localip.cc");
    /// assert_eq!(Request::new().uri("http://fibra.api.localip.cc").domain(), "localip.cc");
    /// assert_eq!(Request::new().uri("https://www.google.com.hk").domain(), "google.com.hk");
    /// ```
    #[inline]
    pub fn domain(&self) -> &str {
        match psl::domain(self.host().as_bytes()) {
            Some(d) => unsafe { std::str::from_utf8_unchecked(d.as_bytes()) },
            None => "",
        }
    }

    /// Get the subdomain from the host
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().subdomain(), "");
    /// assert_eq!(Request::new().uri("http://localip.cc").subdomain(), "");
    /// assert_eq!(Request::new().uri("http://www.localip.cc").subdomain(), "www");
    /// assert_eq!(Request::new().uri("http://fibra.api.localip.cc").subdomain(), "fibra.api");
    /// assert_eq!(Request::new().uri("https://www.google.com.hk").subdomain(), "www");
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().host(), "");
    ///
    /// assert_eq!(Request::new().uri("http://localip.cc").host(), "localip.cc");
    /// assert_eq!(Request::new().uri("http://localip.cc:3000").host(), "localip.cc");
    /// assert_eq!(Request::new().uri("http://www.localip.cc").host(), "www.localip.cc");
    ///
    /// assert_eq!(Request::new().header(header::HOST, "localip.cc").host(), "localip.cc");
    /// assert_eq!(Request::new().header(header::HOST, "localip.cc:3000").host(), "localip.cc");
    /// assert_eq!(Request::new().header(header::HOST, "www.localip.cc").host(), "www.localip.cc");
    /// ```
    #[inline]
    pub fn host(&self) -> &str {
        if let Some(host) = self.header_ref(header::HOST) {
            if let Ok(host) = std::str::from_utf8(host.as_bytes()) {
                return match host.rfind(':') {
                    None => host,
                    Some(find) => &host[..find],
                };
            }
        }

        self.uri.host().unwrap_or("")
    }

    /// Get the port
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().port(), 80);
    ///
    /// assert_eq!(Request::new().uri("http://localip.cc").port(), 80);
    /// assert_eq!(Request::new().uri("http://localip.cc:3000").port(), 3000);
    /// assert_eq!(Request::new().uri("https://localip.cc").port(), 443);
    /// assert_eq!(Request::new().uri("https://www.localip.cc:8443").port(), 8443);
    ///
    /// assert_eq!(Request::new().header(header::HOST, "localip.cc").port(), 80);
    /// assert_eq!(Request::new().header(header::HOST, "localip.cc:3000").port(), 3000);
    /// ```
    #[inline]
    pub fn port(&self) -> u16 {
        if let Some(host) = self.header_ref(header::HOST) {
            if let Ok(host) = std::str::from_utf8(host.as_bytes()) {
                if let Some(find) = host.rfind(':') {
                    return host[find + 1..].parse().unwrap_or(0)
                }
            }
        }

        match self.uri.port_u16() {
            Some(port) => port,
            None => match self.scheme() {
                Scheme::HTTP => 80,
                Scheme::HTTPS => 443,
            }
        }
    }

    /// Get the path
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().path(), "/");
    /// assert_eq!(Request::new().uri("http://localip.cc").path(), "/");
    /// assert_eq!(Request::new().uri("http://localip.cc/").path(), "/");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog").path(), "/blog");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog/2024").path(), "/blog/2024");
    /// assert_eq!(Request::new().uri("http://localip.cc:3000").path(), "/");
    /// assert_eq!(Request::new().uri("http://localip.cc:3000/").path(), "/");
    /// assert_eq!(Request::new().uri("http://localip.cc:3000/blog").path(), "/blog");
    /// assert_eq!(Request::new().uri("http://localip.cc:3000/blog/2024").path(), "/blog/2024");
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().query(), "");
    /// assert_eq!(Request::new().uri("http://localip.cc").query(), "");
    /// assert_eq!(Request::new().uri("http://localip.cc/").query(), "");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog").query(), "");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog?").query(), "");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog?nonce=1a2b3c").query(), "nonce=1a2b3c");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog?nonce=1a2b3c&signature=abcde").query(), "nonce=1a2b3c&signature=abcde");
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
    /// use fibra::*;
    ///
    /// assert_eq!(Request::new().href(), "/");
    /// assert_eq!(Request::new().uri("http://localip.cc").href(), "http://localip.cc/");
    /// assert_eq!(Request::new().uri("http://localip.cc/").href(), "http://localip.cc/");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog").href(), "http://localip.cc/blog");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog?").href(), "http://localip.cc/blog?");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog?nonce=1a2b3c").href(), "http://localip.cc/blog?nonce=1a2b3c");
    /// assert_eq!(Request::new().uri("http://localip.cc/blog?nonce=1a2b3c&signature=abcde").href(), "http://localip.cc/blog?nonce=1a2b3c&signature=abcde");
    /// ```
    #[inline]
    pub fn href(&self) -> String {
        self.uri.to_string()
    }
}

impl Default for Request {
    #[inline]
    fn default() -> Self {
        Self {
            created: SystemTime::now(),
            method: Default::default(),
            uri: Default::default(),
            version: Default::default(),
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

impl From<hyper::Request<hyper::body::Incoming>> for Request {
    #[inline]
    fn from(from: hyper::Request<hyper::body::Incoming>) -> Self {
        use http_body_util::BodyExt;

        let (head, body) = from.into_parts();
        Self {
            created: SystemTime::now(),
            method: head.method,
            uri: head.uri,
            version: head.version,
            headers: head.headers,
            body: body.map_err(|err| err.into()).boxed().into(),
        }
    }
}

impl From<hyper::http::request::Parts> for Request {
    #[inline]
    fn from(head: hyper::http::request::Parts) -> Self {
        Self {
            created: SystemTime::now(),
            method: head.method,
            uri: head.uri,
            version: head.version,
            headers: head.headers,
            body: Default::default(),
        }
    }
}
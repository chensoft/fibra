//! HTTP Response
use crate::types::*;

/// HTTP Response
#[derive(Default)]
pub struct Response {
    /// HTTP Version
    version: Version,

    /// Status Code
    status: Status,

    /// HTTP Headers
    headers: HeaderMap,

    /// Response Body
    body: Body
}

impl Response {
    /// Create a new object
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the http version
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Response::new().version_ref(), &Version::HTTP_11);
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
    /// let mut res = Response::new();
    /// *res.version_mut() = Version::HTTP_10;
    /// assert_eq!(res.version_mut(), &Version::HTTP_10);
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
    /// let mut res = Response::new().version(Version::HTTP_10);
    /// assert_eq!(res.version_mut(), &Version::HTTP_10);
    /// ```
    #[inline]
    pub fn version(mut self, val: impl Into<Version>) -> Self {
        self.version = val.into();
        self
    }

    /// Get the status code
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Response::new().status_ref(), &Status::OK);
    /// ```
    #[inline]
    pub fn status_ref(&self) -> &Status {
        &self.status
    }

    /// Get/Set the status code
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut res = Response::new();
    /// *res.status_mut() = Status::NOT_FOUND;
    /// assert_eq!(res.status_mut(), &Status::NOT_FOUND);
    /// ```
    #[inline]
    pub fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    /// Set the status code
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut res = Response::new().status(Status::NOT_FOUND);
    /// assert_eq!(res.status_mut(), &Status::NOT_FOUND);
    /// ```
    #[inline]
    pub fn status(mut self, val: impl Into<Status>) -> Self {
        self.status = val.into();
        self
    }

    /// Get the response headers
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Response::new().headers_ref().len(), 0);
    /// ```
    #[inline]
    pub fn headers_ref(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get/Set the response headers
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut res = Response::new();
    /// res.headers_mut().insert(header::CONTENT_TYPE, mime::APPLICATION_JSON.into_header_value());
    /// assert_eq!(res.headers_mut()[header::CONTENT_TYPE], mime::APPLICATION_JSON.as_ref());
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Set the response headers
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut map = HeaderMap::new();
    /// let mut res = Response::new();
    ///
    /// map.insert(header::CONTENT_TYPE, mime::APPLICATION_JSON.into_header_value());
    /// res = res.headers(map);
    ///
    /// assert_eq!(res.headers_mut()[header::CONTENT_TYPE], mime::APPLICATION_JSON.as_ref());
    /// ```
    #[inline]
    pub fn headers(mut self, val: impl Into<HeaderMap>) -> Self {
        self.headers = val.into();
        self
    }

    /// Get a response header's value
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut res = Response::new();
    /// res = res.header(header::CONTENT_TYPE, mime::APPLICATION_JSON);
    ///
    /// assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
    /// ```
    #[inline]
    pub fn header_ref(&self, key: impl AsHeaderName) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    /// Get/Set a response header's value
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut res = Response::new().header(header::CONTENT_TYPE, mime::APPLICATION_JSON);
    /// res.header_mut(header::CONTENT_TYPE).map(|v| *v = mime::TEXT_PLAIN_UTF_8.into_header_value());
    ///
    /// assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::TEXT_PLAIN_UTF_8.as_ref().as_bytes()));
    /// ```
    #[inline]
    pub fn header_mut(&mut self, key: impl AsHeaderName) -> Option<&mut HeaderValue> {
        self.headers.get_mut(key)
    }

    /// Set a response header's value
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut res = Response::new().header(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8);
    ///
    /// assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::TEXT_HTML_UTF_8.as_ref().as_bytes()));
    /// ```
    #[inline]
    pub fn header(mut self, key: impl IntoHeaderName, val: impl IntoHeaderValue) -> Self {
        self.headers.insert(key.into_header_name(), val.into_header_value());
        self
    }

    /// Get the http body
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// Response::new().body_ref();
    /// ```
    #[inline]
    pub fn body_ref(&self) -> &Body {
        &self.body
    }

    /// Get/Set the http body
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
    ///     let mut res = Response::new().body("Hello World!");
    ///     assert_eq!(res.body_all().await?, "Hello World!");
    ///     Ok(())
    /// }
    /// ```
    pub async fn body_all(&mut self) -> FibraResult<Bytes> {
        use body::HttpBody;
        Ok(self.body_mut().collect().await?.to_bytes())
    }

    /// Set the http body without predefined content-type
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut res = Response::new().body("Hello World!");
    ///     assert_eq!(res.body_all().await?, "Hello World!");
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn body(mut self, val: impl Into<Body>) -> Self {
        self.body = val.into();
        self
    }

    /// Set JSON response with correct content-type header
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use indexmap::indexmap;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let map = indexmap!(
    ///         "a" => 1,
    ///         "b" => 2,
    ///     );
    ///
    ///     let mut res = Response::new().json(map);
    ///
    ///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
    ///     assert_eq!(res.body_all().await?, "{\"a\":1,\"b\":2}");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn json(self, val: impl encoder::json::Encode) -> Self {
        let mut buf = vec![];
        val.encode(&mut buf);
        self.header(header::CONTENT_TYPE, mime::APPLICATION_JSON).body(buf)
    }

    /// Set JSONP response with a callback name
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use indexmap::indexmap;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let map = indexmap!(
    ///         "a" => 1,
    ///         "b" => 2,
    ///     );
    ///
    ///     let mut res = Response::new().jsonp(map, "callback");
    ///
    ///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
    ///     assert_eq!(res.body_all().await?, "callback({\"a\":1,\"b\":2})");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn jsonp(self, val: impl encoder::json::Encode, callback: &str) -> Self {
        let mut buf = vec![];
        buf.extend(callback.as_bytes());
        buf.push(b'(');
        val.encode(&mut buf);
        buf.push(b')');

        self.header(header::CONTENT_TYPE, mime::APPLICATION_JSON).body(buf)
    }

    /// Set plain text response with TEXT_PLAIN_UTF_8 content-type header
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut res = Response::new().text("It Works!");
    ///
    ///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::TEXT_PLAIN_UTF_8.as_ref().as_bytes()));
    ///     assert_eq!(res.body_all().await?, "It Works!");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn text(self, val: impl Into<String>) -> Self {
        self.header(header::CONTENT_TYPE, mime::TEXT_PLAIN_UTF_8).body(val.into())
    }

    /// Set plain text response with TEXT_HTML_UTF_8 content-type header
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut res = Response::new().html("<html><body>It Works!</body></html>");
    ///
    ///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::TEXT_HTML_UTF_8.as_ref().as_bytes()));
    ///     assert_eq!(res.body_all().await?, "<html><body>It Works!</body></html>");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn html(self, val: impl Into<Body>) -> Self {
        self.header(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8).body(val.into())
    }

    /// Set raw byte stream response with APPLICATION_OCTET_STREAM
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut res = Response::new().bytes(b"abc".to_vec());
    ///
    ///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_OCTET_STREAM.as_ref().as_bytes()));
    ///     assert_eq!(res.body_all().await?, b"abc".to_vec());
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn bytes(self, val: impl Into<Body>) -> Self {
        self.header(header::CONTENT_TYPE, mime::APPLICATION_OCTET_STREAM).body(val)
    }

    /// Set custom stream response without predefined content-type
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use futures::Stream;
    /// use std::task::Poll;
    /// use std::io::{BufReader, Read};
    ///
    /// struct FileStream(BufReader<std::fs::File>);
    ///
    /// impl FileStream {
    ///     pub fn new() -> FibraResult<Self> {
    ///         std::fs::write(std::env::temp_dir().join("sample.txt"), "The quick brown fox jumps over the lazy dog.")?;
    ///         Ok(Self(BufReader::new(std::fs::File::open(std::env::temp_dir().join("sample.txt"))?)))
    ///     }
    /// }
    ///
    /// impl Stream for FileStream {
    ///     type Item = FibraResult<Vec<u8>>;
    ///
    ///     fn poll_next(mut self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    ///         let mut buffer = vec![0; 10];
    ///         match self.0.read(&mut buffer) {
    ///             Ok(0) => Poll::Ready(None),
    ///             Ok(n) => {
    ///                 buffer.truncate(n);
    ///                 Poll::Ready(Some(Ok(buffer)))
    ///             },
    ///             Err(e) => Poll::Ready(Some(Err(e.into()))),
    ///         }
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let mut res = Response::new().stream(FileStream::new()?);
    ///
    ///     assert_eq!(res.body_all().await?, "The quick brown fox jumps over the lazy dog.");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn stream<S, O>(self, val: S) -> Self
        where
            S: Stream<Item = FibraResult<O>> + Send + 'static,
            O: Into<Bytes> + 'static,
    {
        self.body(Body::wrap_stream(val))
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = (Status::NOT_FOUND, mime::TEXT_PLAIN_UTF_8, "Not Found").into();
///
///     assert_eq!(res.status_ref(), &Status::NOT_FOUND);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some("text/plain; charset=utf-8".as_bytes()));
///     assert_eq!(res.body_all().await?, "Not Found");
///
///     Ok(())
/// }
/// ```
impl<T> From<(Status, Mime, T)> for Response
    where T: Into<Body>
{
    #[inline]
    fn from((status, mime, body): (Status, Mime, T)) -> Self {
        Response::new().status(status).header(header::CONTENT_TYPE, mime).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = (Status::NOT_FOUND, "Not Found").into();
///
///     assert_eq!(res.status_ref(), &Status::NOT_FOUND);
///     assert_eq!(res.body_all().await?, "Not Found");
///
///     Ok(())
/// }
/// ```
impl From<(Status, &'static str)> for Response {
    #[inline]
    fn from((status, body): (Status, &'static str)) -> Self {
        Response::new().status(status).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = (Status::NOT_FOUND, "Not Found".to_string()).into();
///
///     assert_eq!(res.status_ref(), &Status::NOT_FOUND);
///     assert_eq!(res.body_all().await?, "Not Found");
///
///     Ok(())
/// }
/// ```
impl From<(Status, String)> for Response {
    #[inline]
    fn from((status, body): (Status, String)) -> Self {
        Response::new().status(status).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = (Status::FORBIDDEN, b"Forbidden".as_slice()).into();
///
///     assert_eq!(res.status_ref(), &Status::FORBIDDEN);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some("application/octet-stream".as_bytes()));
///     assert_eq!(res.body_all().await?, b"Forbidden".as_slice());
///
///     Ok(())
/// }
/// ```
impl From<(Status, &'static [u8])> for Response {
    #[inline]
    fn from((status, body): (Status, &'static [u8])) -> Self {
        Response::new().status(status).header(header::CONTENT_TYPE, mime::APPLICATION_OCTET_STREAM).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = (Status::FORBIDDEN, b"Forbidden".to_vec()).into();
///
///     assert_eq!(res.status_ref(), &Status::FORBIDDEN);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some("application/octet-stream".as_bytes()));
///     assert_eq!(res.body_all().await?, b"Forbidden".to_vec());
///
///     Ok(())
/// }
/// ```
impl From<(Status, Vec<u8>)> for Response {
    #[inline]
    fn from((status, body): (Status, Vec<u8>)) -> Self {
        Response::new().status(status).header(header::CONTENT_TYPE, mime::APPLICATION_OCTET_STREAM).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = (mime::TEXT_PLAIN_UTF_8, "OK").into();
///
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some("text/plain; charset=utf-8".as_bytes()));
///     assert_eq!(res.body_all().await?, "OK");
///
///     Ok(())
/// }
/// ```
impl<T> From<(Mime, T)> for Response
    where T: Into<Body>
{
    #[inline]
    fn from((mime, body): (Mime, T)) -> Self {
        Response::new().header(header::CONTENT_TYPE, mime).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = ().into();
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE), None);
///     assert_eq!(res.body_all().await?, "");
///
///     Ok(())
/// }
/// ```
impl From<()> for Response {
    #[inline]
    fn from(_: ()) -> Self {
        Response::new()
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = Status::INTERNAL_SERVER_ERROR.into();
///
///     assert_eq!(res.status_ref(), &Status::INTERNAL_SERVER_ERROR);
///
///     Ok(())
/// }
/// ```
impl From<Status> for Response {
    #[inline]
    fn from(status: Status) -> Self {
        Response::new().status(status)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = "Hello World!".into();
///
///     assert_eq!(res.body_all().await?, "Hello World!");
///
///     Ok(())
/// }
/// ```
impl From<&'static str> for Response {
    #[inline]
    fn from(body: &'static str) -> Self {
        Response::new().body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = "Hello World!".to_string().into();
///
///     assert_eq!(res.body_all().await?, "Hello World!");
///
///     Ok(())
/// }
/// ```
impl From<String> for Response {
    #[inline]
    fn from(body: String) -> Self {
        Response::new().body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = b"Hello World!".as_slice().into();
///
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some("application/octet-stream".as_bytes()));
///     assert_eq!(res.body_all().await?, "Hello World!");
///
///     Ok(())
/// }
/// ```
impl From<&'static [u8]> for Response {
    #[inline]
    fn from(body: &'static [u8]) -> Self {
        Response::new().header(header::CONTENT_TYPE, mime::APPLICATION_OCTET_STREAM).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let mut res: Response = b"Hello World!".to_vec().into();
///
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some("application/octet-stream".as_bytes()));
///     assert_eq!(res.body_all().await?, "Hello World!");
///
///     Ok(())
/// }
/// ```
impl From<Vec<u8>> for Response {
    #[inline]
    fn from(body: Vec<u8>) -> Self {
        Response::new().header(header::CONTENT_TYPE, mime::APPLICATION_OCTET_STREAM).body(body)
    }
}

/// Conversion
///
/// # Examples
///
/// ```
/// use fibra::*;
/// use body::HttpBody;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let raw: Response = (Status::NOT_FOUND, mime::TEXT_PLAIN_UTF_8, Status::NOT_FOUND.canonical_reason().unwrap_or("")).into();
///     let mut res: hyper::Response<Body> = raw.into();
///
///     assert_eq!(res.status(), Status::NOT_FOUND.as_u16());
///     assert_eq!(res.headers().get(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some("text/plain; charset=utf-8".as_bytes()));
///     assert_eq!(res.body_mut().collect().await?.to_bytes(), "Not Found");
///
///     Ok(())
/// }
/// ```
impl From<Response> for hyper::Response<Body> {
    #[inline]
    fn from(value: Response) -> Self {
        let mut res = hyper::Response::default();
        *res.version_mut() = value.version;
        *res.status_mut() = value.status;
        *res.headers_mut() = value.headers;
        *res.body_mut() = value.body;
        res
    }
}
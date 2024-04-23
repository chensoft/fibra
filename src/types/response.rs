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
    /// ```
    /// use fibra::{Response, Version};
    ///
    /// assert_eq!(Response::default().version_ref(), &Version::HTTP_11);
    /// ```
    pub fn version_ref(&self) -> &Version {
        &self.version
    }

    /// ```
    /// use fibra::{Response, Version};
    ///
    /// let mut res = Response::default();
    /// *res.version_mut() = Version::HTTP_10;
    /// assert_eq!(res.version_mut(), &Version::HTTP_10);
    /// ```
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    /// ```
    /// use fibra::{Response, Version};
    ///
    /// let mut res = Response::default().version(Version::HTTP_10);
    /// assert_eq!(res.version_mut(), &Version::HTTP_10);
    /// ```
    pub fn version(mut self, val: impl Into<Version>) -> Self {
        self.version = val.into();
        self
    }

    /// ```
    /// use fibra::{Response, Status};
    ///
    /// assert_eq!(Response::default().status_ref(), &Status::OK);
    /// ```
    pub fn status_ref(&self) -> &Status {
        &self.status
    }

    /// ```
    /// use fibra::{Response, Status};
    ///
    /// let mut res = Response::default();
    /// *res.status_mut() = Status::NOT_FOUND;
    /// assert_eq!(res.status_mut(), &Status::NOT_FOUND);
    /// ```
    pub fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    /// ```
    /// use fibra::{Response, Status};
    ///
    /// let mut res = Response::default().status(Status::NOT_FOUND);
    /// assert_eq!(res.status_mut(), &Status::NOT_FOUND);
    /// ```
    pub fn status(mut self, val: impl Into<Status>) -> Self {
        self.status = val.into();
        self
    }

    /// ```
    /// use fibra::{Response};
    ///
    /// assert_eq!(Response::default().headers_ref().len(), 0);
    /// ```
    pub fn headers_ref(&self) -> &HeaderMap {
        &self.headers
    }

    /// ```
    /// use fibra::{Response, header::{self, IntoHeaderValue}};
    ///
    /// let mut res = Response::default();
    /// res.headers_mut().insert(header::CONTENT_TYPE, mime::APPLICATION_JSON.into_value());
    /// assert_eq!(res.headers_mut()[header::CONTENT_TYPE], mime::APPLICATION_JSON.into_value());
    /// ```
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// ```
    /// use fibra::{Response, header::{self, HeaderMap, IntoHeaderValue}};
    ///
    /// let mut map = HeaderMap::new();
    /// let mut res = Response::default();
    ///
    /// map.insert(header::CONTENT_TYPE, mime::APPLICATION_JSON.into_value());
    /// res = res.headers(map);
    ///
    /// assert_eq!(res.headers_mut()[header::CONTENT_TYPE], mime::APPLICATION_JSON.into_value());
    /// ```
    pub fn headers(mut self, val: HeaderMap) -> Self {
        self.headers = val;
        self
    }

    /// ```
    /// use fibra::{Response, header::{self, IntoHeaderValue}};
    ///
    /// let mut res = Response::default();
    /// res = res.header(header::CONTENT_TYPE, mime::APPLICATION_JSON);
    ///
    /// assert_eq!(res.header_ref(header::CONTENT_TYPE), Some(&mime::APPLICATION_JSON.into_value()));
    /// ```
    pub fn header_ref(&self, key: impl AsHeaderName) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    /// ```
    /// use fibra::{Response, header::{self, IntoHeaderValue}};
    ///
    /// let mut res = Response::default().header(header::CONTENT_TYPE, mime::APPLICATION_JSON);
    /// res.header_mut(header::CONTENT_TYPE).map(|v| *v = mime::TEXT_PLAIN_UTF_8.into_value());
    ///
    /// assert_eq!(res.header_ref(header::CONTENT_TYPE), Some(&mime::TEXT_PLAIN_UTF_8.into_value()));
    /// ```
    pub fn header_mut(&mut self, key: impl header::AsHeaderName) -> Option<&mut HeaderValue> {
        self.headers.get_mut(key)
    }

    /// ```
    /// use fibra::{Response, header::{self, IntoHeaderValue}};
    ///
    /// let mut res = Response::default().header(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8);
    ///
    /// assert_eq!(res.header_ref(header::CONTENT_TYPE), Some(&mime::TEXT_HTML_UTF_8.into_value()));
    /// ```
    pub fn header(mut self, key: impl IntoHeaderName, val: impl IntoHeaderValue) -> Self {
        self.headers.insert(key, val.into_value());
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

    pub fn json(self, val: impl encoder::json::Encode) -> Self {
        let mut buf = vec![];
        val.encode(&mut buf);
        self.header(header::CONTENT_TYPE, mime::APPLICATION_JSON).body(buf)
    }

    pub fn jsonp(self, val: impl encoder::json::Encode, callback: &str) -> Self {
        let mut buf = vec![];
        buf.extend(callback.as_bytes());
        buf.push(b'(');
        val.encode(&mut buf);
        buf.push(b')');

        self.header(header::CONTENT_TYPE, mime::APPLICATION_JSON).body(buf)
    }

    pub fn text(self, val: impl Into<String>) -> Self {
        self.header(header::CONTENT_TYPE, mime::TEXT_PLAIN_UTF_8).body(val.into())
    }

    pub fn text_add(self, _val: &str) -> Self {
        todo!()
    }

    pub fn html(self) -> Self {
        self.header(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8) // todo template engine
    }

    pub fn file(self) -> Self {
        todo!() // auto detect file mime, chunk transfer, stream wrap attachment header
    }

    pub fn bytes(self, val: impl Into<Body>) -> Self {
        self.header(header::CONTENT_TYPE, mime::APPLICATION_OCTET_STREAM).body(val)
    }

    pub fn bytes_add(self, _val: impl Into<Body>) -> Self {
        self.header(header::CONTENT_TYPE, mime::APPLICATION_OCTET_STREAM) // todo
    }

    /// ```
    /// use bytes::Bytes;
    /// use futures::Stream;
    /// use std::task::Poll;
    /// use std::io::{BufReader, Read};
    /// use fibra::{Response, FibraResult};
    ///
    /// struct FileStream(BufReader<std::fs::File>);
    ///
    /// impl FileStream {
    ///     pub fn new() -> FibraResult<Self> {
    ///         std::fs::write(std::env::temp_dir().join("sample.txt"), "Actions speak louder than words")?;
    ///         Ok(Self(BufReader::new(std::fs::File::open(std::env::temp_dir().join("sample.txt"))?)))
    ///     }
    /// }
    ///
    /// impl Stream for FileStream {
    ///     type Item = FibraResult<Bytes>;
    ///
    ///     fn poll_next(mut self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    ///         let mut buffer = vec![0; 10];
    ///         match self.0.read(&mut buffer) {
    ///             Ok(0) => Poll::Ready(None),
    ///             Ok(n) => {
    ///                 buffer.truncate(n);
    ///                 Poll::Ready(Some(Ok(Bytes::from(buffer))))
    ///             },
    ///             Err(e) => Poll::Ready(Some(Err(e.into()))),
    ///         }
    ///     }
    /// }
    ///
    /// fn main() -> FibraResult<()> {
    ///     let _ = Response::default().stream(FileStream::new()?);
    ///     Ok(())
    /// }
    /// ```
    pub fn stream<S, O>(self, val: S) -> Self
        where
            S: Stream<Item = FibraResult<O>> + Send + 'static,
            O: Into<Bytes> + 'static,
    {
        self.body(Body::wrap_stream(val))
    }
}

impl<T> From<(Status, Mime, T)> for Response
    where T: Into<Body>
{
    fn from((status, mime, body): (Status, Mime, T)) -> Self {
        Self::default().status(status).header(header::CONTENT_TYPE, mime).body(body)
    }
}

impl From<(Status, &'static str)> for Response {
    fn from((status, body): (Status, &'static str)) -> Self {
        (status, mime::TEXT_PLAIN_UTF_8, body).into()
    }
}

impl From<(Status, String)> for Response {
    fn from((status, body): (Status, String)) -> Self {
        (status, mime::TEXT_PLAIN_UTF_8, body).into()
    }
}

impl From<(Status, &'static [u8])> for Response {
    fn from((status, body): (Status, &'static [u8])) -> Self {
        (status, mime::APPLICATION_OCTET_STREAM, body).into()
    }
}

impl From<(Status, Vec<u8>)> for Response {
    fn from((status, body): (Status, Vec<u8>)) -> Self {
        (status, mime::APPLICATION_OCTET_STREAM, body).into()
    }
}

impl From<()> for Response {
    fn from(_: ()) -> Self {
        (Status::OK, mime::TEXT_PLAIN_UTF_8, "").into()
    }
}

impl From<Status> for Response {
    fn from(status: Status) -> Self {
        (status, mime::TEXT_PLAIN_UTF_8, "").into()
    }
}

impl From<&'static str> for Response {
    fn from(body: &'static str) -> Self {
        (Status::OK, mime::TEXT_PLAIN_UTF_8, body).into()
    }
}

impl From<String> for Response {
    fn from(body: String) -> Self {
        (Status::OK, mime::TEXT_PLAIN_UTF_8, body).into()
    }
}

impl From<&'static [u8]> for Response {
    fn from(body: &'static [u8]) -> Self {
        (Status::OK, mime::APPLICATION_OCTET_STREAM, body).into()
    }
}

impl From<Vec<u8>> for Response {
    fn from(body: Vec<u8>) -> Self {
        (Status::OK, mime::APPLICATION_OCTET_STREAM, body).into()
    }
}

impl Into<hyper::Response<Body>> for Response {
    fn into(self) -> hyper::Response<Body> {
        let mut res = hyper::Response::default();
        *res.version_mut() = self.version;
        *res.status_mut() = self.status;
        *res.headers_mut() = self.headers;
        *res.body_mut() = self.body;
        res
    }
}
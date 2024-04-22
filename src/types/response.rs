use crate::types::*;

#[derive(Default)]
pub struct Response {
    version: Version,
    status: Status,
    headers: HeaderMap,
    body: Body
}

impl Response {
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

    pub fn status_ref(&self) -> &Status {
        &self.status
    }

    pub fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    pub fn status(mut self, val: impl Into<Status>) -> Self {
        self.status = val.into();
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

    pub fn header(mut self, key: impl header::IntoHeaderName, val: impl header::IntoHeaderValue) -> Self {
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

    pub fn stream(self) -> Self {
        todo!()
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
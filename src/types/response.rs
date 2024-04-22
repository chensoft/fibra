use crate::types::*;

pub struct Response {
    res: hyper::Response<Body>,
}

impl Response {
    pub fn status_ref(&self) -> &Status { todo!() }
    pub fn status_mut(&mut self) {}
    pub fn status(mut self, val: Status) -> Self {
        *self.res.status_mut() = val;
        self
    }

    pub fn version_ref(&self) -> &Version { todo!() }
    pub fn version_mut(&mut self) {}
    pub fn version(mut self, val: Version) -> Self {
        *self.res.version_mut() = val;
        self
    }

    pub fn headers_ref(&self) {}
    pub fn headers_mut(&mut self) {}
    pub fn headers(&mut self) {}

    pub fn header_ref(&self) {}
    pub fn header_mut(&mut self) {}
    pub fn header(self, key: HeaderName, val: HeaderValue) -> Self { // todo Cow val
        self
    }

    pub fn body_ref(&self) {}
    pub fn body_mut(&mut self) {}
    pub fn body(self, val: impl Into<Body>) -> Self {
        self // binary with oct
    }

    pub fn json(self) -> Self {
        self
    }

    pub fn jsonp(self, callback: &str) -> Self {
        self
    }

    pub fn text(self, val: &str) -> Self {
        self // Cow
    }
    pub fn text_add(self, val: &str) -> Self {
        self // Cow
    }

    pub fn html(self) -> Self {
        self // template engine
    }

    pub fn file(self) -> Self {
        self // auto detect file mime, chunk transfer, stream wrap attachment header
    }

    pub fn bytes(self, val: impl Into<Body>) -> Self {
        self // binary with oct
    }
    pub fn bytes_add(self, val: impl Into<Body>) -> Self {
        self // binary with oct
    }

    pub fn steam(self) -> Self {
        self
    }
}

impl Default for Response {
    fn default() -> Self {
        todo!()
    }
}

impl Into<hyper::Response<Body>> for Response {
    fn into(self) -> hyper::Response<Body> {
        todo!()
    }
}
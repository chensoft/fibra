use crate::types::*;

pub struct Response {
    res: hyper::Response<Body>,
}

impl Response {
    pub fn status_ref(&self) -> &Status { todo!() }
    pub fn status_mut(&mut self) {}
    pub fn status(mut self, code: Status) -> Self {
        *self.res.status_mut() = code;
        self
    }

    pub fn header_ref(&self) {}
    pub fn header_mut(&mut self) {}
    pub fn header(self, key: HeaderName, val: HeaderValue) -> Self { // todo Cow val
        self
    }

    pub fn headers_ref(&self) {}
    pub fn headers_mut(&mut self) {}

    pub fn body_ref(&self) {}
    pub fn body_mut(&mut self) {}
    pub fn body(self, data: impl Into<Body>) -> Self {
        self // binary with oct
    }

    pub fn json_ref(&self) {}
    pub fn json_mut(&mut self) {}
    pub fn json(self) -> Self {
        self
    }

    pub fn jsonp_ref(&self) {}
    pub fn jsonp_mut(&mut self) {}
    pub fn jsonp(self, callback: &str) -> Self {
        self
    }

    pub fn text_ref(&self) {}
    pub fn text_mut(&mut self) {}
    pub fn text(self, data: &str) -> Self {
        self // Cow
    }

    pub fn html_ref(&self) {}
    pub fn html_mut(&mut self) {}
    pub fn html(self) -> Self {
        self // template engine
    }

    pub fn file_ref(&self) {}
    pub fn file_mut(&mut self) {}
    pub fn file(self) -> Self {
        self // auto detect file mime, chunk transfer, stream wrap attachment header
    }

    pub fn stream_ref(&self) {}
    pub fn stream_mut(&mut self) {}
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
use crate::types::*;

pub trait Replies {
    fn set_status(self, status: StatusCode) -> Self;

    // todo more headers
    fn set_header(self, key: HeaderName, val: HeaderValue) -> Self;
    fn set_content_type(self, mime: Mime) -> Self;

    fn set_body(self, body: impl Into<Body>) -> Self; // binary with oct
    fn set_file(self) -> Self; // auto detect file mime, chunk transfer, stream wrap attachment header
    fn set_text(self, text: &str) -> Self; // Cow
    fn set_html(self) -> Self; // template engine
    fn set_json(self) -> Self;
    fn set_jsonp(self, callback: &str) -> Self;
    fn set_stream(self) -> Self;
}

impl Replies for Response<Body> {
    fn set_status(self, status: StatusCode) -> Self {
        todo!()
    }

    fn set_header(self, key: HeaderName, val: HeaderValue) -> Self {
        todo!()
    }

    fn set_content_type(self, mime: Mime) -> Self {
        todo!()
    }

    fn set_body(self, body: impl Into<Body>) -> Self {
        todo!()
    }

    fn set_file(self) -> Self {
        todo!()
    }

    fn set_text(self, _text: &str) -> Self {
        todo!()
    }

    fn set_html(self) -> Self {
        todo!()
    }

    fn set_json(self) -> Self {
        todo!()
    }

    fn set_jsonp(self, callback: &str) -> Self {
        todo!()
    }

    fn set_stream(self) -> Self {
        todo!()
    }
}
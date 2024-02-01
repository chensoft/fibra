use super::types::*;
use super::cache::*;

pub struct Context {
    pub req: http::Request<http::Body>,
    pub res: http::Response<http::Body>,
    pub peer: SocketAddr,
    pub temp: Cache,
}

impl Drop for Context {
    fn drop(&mut self) {
        // todo reuse
    }
}

impl Context {
    pub fn new(_req: http::Request<http::Body>) -> Self {
        todo!()
    }

    pub fn next(&mut self) {}
    pub fn abort(self) {}
    pub fn param(&mut self, _key: &str) {}
    pub fn rewrite(self, _to: Cow<'static, str>) {}
    pub fn redirect(self, _to: Cow<'static, str>, _code: http::StatusCode) {}
}
use crate::http;
use super::cache::*;
use super::define::*;

pub struct Context {
    pub cache: Cache,
    pub cookie: http::Cookie,
    pub req: http::Request,
    pub res: http::Response,
}

impl Drop for Context {
    fn drop(&mut self) {
        // todo reuse
    }
}

impl Context {
    pub fn new(_req: http::Request) -> Self {
        todo!()
    }

    pub fn next(&mut self) {}
    pub fn abort(self) {}
    pub fn param(&mut self, _key: &str) {}
    pub fn rewrite(self, _to: Cow<'static, str>) {}
    pub fn redirect(self, _to: Cow<'static, str>, _code: http::StatusCode) {}
}
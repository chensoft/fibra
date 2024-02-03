use super::general::*;
use super::storage::*;

pub struct Context {
    pub req: http::Request<http::Body>,
    pub res: http::Response<http::Body>,
    pub sock: SocketAddr,
    pub peer: SocketAddr,
    pub temp: Storage,
}

// impl Drop for Context {
//     fn drop(&mut self) {
//         todo reuse
    // }
// }

impl Context {
    pub fn new(_req: http::Request<http::Body>) -> Self {
        todo!()
    }

    pub fn next(&mut self) -> Result<()> { Ok(()) }
    pub fn abort(self) -> Result<()> { Ok(()) }
    pub fn param(&mut self, _key: &str) {}
    pub fn rewrite(self, _to: Cow<'static, str>) {}
    pub fn redirect(self, _to: Cow<'static, str>, _code: http::StatusCode) {}
}
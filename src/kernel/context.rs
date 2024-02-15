use crate::consts::*;
use super::storage::Storage;

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
    pub fn reset(&mut self) {}

    pub async fn next(&mut self) -> Result<()> { Ok(()) }
    pub fn abort(self) -> Result<()> { Ok(()) }
    pub fn param(&mut self, _key: &str) {}
    pub fn rewrite(self, _to: impl Into<http::Uri>) {}
    pub fn redirect(self, _to: impl Into<http::Uri>, _code: http::StatusCode) {}
}
use crate::consts::*;
use crate::veloce::*;
use crate::kernel::*;

pub struct Context {
    pub app: Arc<Veloce>,
    pub req: http::Request<http::Body>,
    pub res: http::Response<http::Body>,
    pub sock: SocketAddr,
    pub peer: SocketAddr,
    pub cache: Storage,
    // pub stack: Vec<>,
}

// impl Drop for Context {
//     fn drop(&mut self) {
//         todo reuse
    // }
// }

impl Context {
    pub fn reset(&mut self) {}

    pub async fn next(self) -> Result<Self> {
        Ok(self)
    }

    pub fn abort(self) -> Result<()> { Ok(()) }
    pub fn param(&self, _key: &str) {}
    pub fn rewrite(self, _to: impl Into<http::Uri>) {}
    pub fn redirect(self, _to: impl Into<http::Uri>, _code: http::StatusCode) {}
}
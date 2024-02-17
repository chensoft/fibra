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
    pub chain: VecDeque<Arc<dyn Handler>>,
}

// impl Drop for Context {
//     fn drop(&mut self) {
//         todo reuse
// }
// }

impl Context {
    pub fn reset(&mut self) {}

    pub async fn next(mut self) -> Result<Self> {
        match self.chain.pop_front() {
            Some(handler) => handler.handle(self).await,
            None => Ok(self),
        }
    }

    pub fn abort(self) -> Result<Self> {
        Err(Error::Aborted.into()) // todo impl in recover
    }

    pub fn param(&self, _key: &str) {}
    pub fn rewrite(self, _to: impl Into<http::Uri>) {}
    pub fn redirect(self, _to: impl Into<http::Uri>, _code: http::StatusCode) {}
}
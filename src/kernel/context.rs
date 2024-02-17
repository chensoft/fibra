use crate::consts::*;
use crate::veloce::*;
use crate::kernel::*;

pub struct Context {
    pub app: Arc<Veloce>,
    pub req: http::Request<http::Body>,
    pub res: http::Response<http::Body>,
    pub sock: SocketAddr,
    pub peer: SocketAddr,
    pub miss: bool,
    pub cache: Storage,
    pub chain: VecDeque<Arc<dyn Handler>>,
}

// impl Drop for Context {
//     fn drop(&mut self) {
//         todo reuse
// }
// }

impl Context {
    pub async fn next(mut self) -> Result<Self> {
        match self.chain.pop_front() {
            Some(handler) => handler.handle(self).await,
            None => Ok(self),
        }
    }

    pub fn abort(self, err: Option<Error>) -> Result<Self> {
        match err {
            None => Err(Error::Aborted.into()),
            Some(val) => Err(val.into()),
        }
    }

    pub fn rewrite(self, _to: impl Into<http::Uri>) { todo!() }
    pub fn redirect(self, _to: impl Into<http::Uri>, _code: http::StatusCode) { todo!() }

    pub fn param(&self, _key: &str) { todo!() }
}
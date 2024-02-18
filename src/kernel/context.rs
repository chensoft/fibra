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

    // todo use replacement
    pub async fn rewrite(mut self, to: http::Uri) -> Result<Self> {
        *self.req.uri_mut() = to;
        self.chain.clear();

        let app = self.app.clone();
        app.handle(self).await
    }

    pub fn redirect(self, _to: &str, _code: http::StatusCode) -> Result<Self> {
        todo!()
    }

    pub fn param(&self, _key: &str) { todo!() }
}
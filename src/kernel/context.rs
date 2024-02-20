use crate::consts::*;
use crate::veloce::*;
use crate::kernel::*;

pub struct Context {
    pub app: Arc<Veloce>,
    pub req: Request<Body>,
    pub res: Response<Body>,

    pub sock: SocketAddr, // local address
    pub peer: SocketAddr, // remote address
    pub temp: Storage,    // temp storage

    pub routes: VecDeque<Arc<dyn Handler>>, // routing chain
    pub parent: String, // parent uri
    pub search: String, // unmatched uri
}

// impl Drop for Context {
//     fn drop(&mut self) {
//         todo reuse
// }
// }

impl Context {
    pub async fn next(mut self) -> Result<Self> {
        match self.routes.pop_front() {
            Some(handler) => handler.handle(self).await,
            None => Ok(self),
        }
    }

    pub fn reject(mut self, status: Option<StatusCode>) -> Result<Self> {
        *self.res.status_mut() = status.unwrap_or(StatusCode::FORBIDDEN);
        Ok(self)
    }

    // todo use replacement, preserve params
    pub async fn rewrite(mut self, to: Uri) -> Result<Self> {
        *self.req.uri_mut() = to;
        self.routes.clear();

        let app = self.app.clone();
        app.handle(self).await
    }

    pub fn redirect(mut self, to: Uri, status: Option<StatusCode>) -> Result<Self> {
        *self.res.status_mut() = status.unwrap_or(StatusCode::TEMPORARY_REDIRECT);
        self.res.headers_mut().insert(header::LOCATION, header::HeaderValue::from_str(to.to_string().as_str())?);
        Ok(self)
    }

    pub fn param(&self, _key: &str) { todo!() }
}
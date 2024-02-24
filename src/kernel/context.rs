use crate::consts::*;
use crate::veloce::*;
use crate::kernel::*;

pub struct Context {
    pub app: Arc<Veloce>,
    pub req: Request<Body>,
    pub res: Response<Body>,

    pub addr: Address, // local & remote addresses
    pub temp: Storage, // temp storage

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
    pub fn new(app: Arc<Veloce>, req: Request<Body>, addr: Address) -> Self {
        let search = req.uri().path().to_string();
        Self {
            app,
            req,
            res: Default::default(),
            addr,
            temp: Default::default(),
            routes: Default::default(),
            parent: "".to_string(),
            search,
        }
    }

    pub async fn next(mut self) -> Result<Context> {
        match self.routes.pop_front() {
            Some(handler) => handler.handle(self).await,
            None => Ok(self),
        }
    }

    pub fn reject(mut self, status: Option<StatusCode>) -> Result<Context> {
        *self.res.status_mut() = status.unwrap_or(StatusCode::FORBIDDEN);
        Ok(self)
    }

    // todo use replacement, preserve params
    pub async fn rewrite(mut self, _to: Uri) -> Result<Context> {
        // *self.req.uri_mut() = to;
        self.routes.clear();

        let app = self.app.clone();
        app.handle(self).await
    }

    pub fn redirect(mut self, to: Uri, status: Option<StatusCode>) -> Result<Context> {
        *self.res.status_mut() = status.unwrap_or(StatusCode::TEMPORARY_REDIRECT);
        self.res.headers_mut().insert(header::LOCATION, header::HeaderValue::from_str(to.to_string().as_str())?);
        Ok(self)
    }

    pub fn param(&self, _key: &str) { todo!() }
}

impl Context {
    pub async fn write_status(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn write_header(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn write_body(&mut self, _multi_type_impl_from: impl Into<String>) -> Result<()> {
        todo!()
    }
}
use crate::consts::*;
use crate::veloce::*;
use crate::kernel::*;

pub struct Context {
    pub app: Arc<Veloce>,
    pub req: Request<Body>,
    pub res: Response<Body>,
    pub nav: Vec<(Arc<Vec<Box<dyn Handler>>>, usize)>,

    pub addr: Address, // local & remote addresses
    pub temp: Storage, // temp storage
}

// impl Drop for Context {
//     fn drop(&mut self) {
//         todo reuse
// }
// }

impl Context {
    pub fn new(app: Arc<Veloce>, req: Request<Body>, addr: Address) -> Self {
        Self {
            app,
            req,
            res: Default::default(),
            nav: vec![],
            addr,
            temp: Default::default(),
        }
    }

    pub fn push(&mut self, routes: Arc<Vec<Box<dyn Handler>>>, index: usize) {
        self.nav.push((routes, index));
    }

    pub async fn next(&mut self) -> Result<()> {
        // match self.routes.pop_front() {
        //     Some(handler) => handler.handle(self).await,
        //     None => Ok(()),
        // }
        todo!()
    }

    pub async fn reject(&mut self, status: Option<StatusCode>) -> Result<()> {
        Err(status.unwrap_or(StatusCode::FORBIDDEN).into_error())
    }

    // todo use replacement, preserve params
    pub async fn rewrite(&mut self, _to: Uri) -> Result<()> {
        // // *self.req.uri_mut() = to;
        // self.routes.clear();
        // 
        // let app = self.app.clone();
        // app.handle(self).await
        todo!()
    }

    pub async fn redirect(&mut self, to: Uri, status: Option<StatusCode>) -> Result<()> {
        *self.res.status_mut() = status.unwrap_or(StatusCode::TEMPORARY_REDIRECT);
        self.res.headers_mut().insert(header::LOCATION, header::HeaderValue::from_str(to.to_string().as_str())?);
        Ok(())
    }
}

impl Context {
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
use crate::consts::*;
use crate::kernel::*;
use crate::veloce::*;

pub struct Context {
    pub app: Arc<Veloce>,
    pub req: Request<Body>,
    pub res: Response<Body>,
    pub nav: Vec<(Arc<Vec<Box<dyn Handler>>>, usize)>,

    pub sock: SocketAddr,
    pub peer: SocketAddr,

    pub cache: Storage,
}

impl Context {
    pub fn new(app: Arc<Veloce>, req: Request<Body>, sock: SocketAddr, peer: SocketAddr) -> Self {
        Self { app, req, res: Default::default(), nav: vec![], sock, peer, cache: Default::default() }
    }

    pub fn push(&mut self, routes: Arc<Vec<Box<dyn Handler>>>, index: usize) {
        self.nav.push((routes, index));
    }

    pub async fn next(&mut self) -> Result<()> {
        while let Some((top, idx)) = self.nav.last_mut() {
            let top = match *idx >= top.len() {
                true => {
                    self.nav.pop();
                    continue;
                }
                false => top.clone(),
            };

            let obj = &top[*idx];
            *idx += 1;
            return obj.handle(self).await;
        }

        Ok(())
    }

    pub async fn reject(&mut self, status: Option<StatusCode>) -> Result<()> {
        Err(status.unwrap_or(StatusCode::FORBIDDEN).into_error())
    }

    pub async fn rewrite(&mut self, to: Uri) -> Result<()> {
        *self.req.uri_mut() = to;

        let app = self.app.clone();
        let mut ctx = Context::new(app.clone(), std::mem::take(&mut self.req), self.sock, self.peer);
        app.handle(&mut ctx).await
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
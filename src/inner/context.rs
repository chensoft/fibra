use crate::inner::*;
use crate::fibra::*;

pub struct Context {
    pub app: Arc<Fibra>,
    pub req: Request<Body>,
    pub res: Response<Body>,

    pub sock: SocketAddr,
    pub peer: SocketAddr,

    pub cache: Storage,
    pub stack: Vec<(*const dyn Handler, usize)>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(app: Arc<Fibra>, req: Request<Body>, sock: SocketAddr, peer: SocketAddr) -> Self {
        Self { app, req, res: Default::default(), sock, peer, cache: Default::default(), stack: vec![] }
    }

    pub fn param(&self, _key: &str) { todo!() }
}

impl Context {
    pub async fn write_status(&mut self) -> FibraResult<()> {
        todo!()
    }

    pub async fn write_header(&mut self) -> FibraResult<()> {
        todo!()
    }

    pub async fn write_body(&mut self, _multi_type_impl_from: impl Into<String>) -> FibraResult<()> {
        todo!()
    }
}

impl Context {
    pub async fn reject(&mut self, status: Option<StatusCode>) -> FibraResult<()> {
        Err(status.unwrap_or(StatusCode::FORBIDDEN).into_error())
    }

    pub async fn rewrite(mut self, to: &'static str) -> FibraResult<Response<Body>> {
        *self.req.uri_mut() = Uri::from_static(to);

        let app = self.app.clone();
        let ctx = Context::new(app.clone(), std::mem::take(&mut self.req), self.sock, self.peer);
        app.handle(ctx).await
    }

    pub async fn redirect(&mut self, to: Uri, status: Option<StatusCode>) -> FibraResult<()> {
        *self.res.status_mut() = status.unwrap_or(StatusCode::TEMPORARY_REDIRECT);
        self.res.headers_mut().insert(header::LOCATION, header::HeaderValue::from_str(to.to_string().as_str())?);
        Ok(())
    }
}

impl Context {
    #[inline]
    pub fn push(&mut self, cur: *const dyn Handler) {
        self.stack.push((cur, 0));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub async fn next(mut self) -> FibraResult<Response<Body>> {
        while let Some((cur, idx)) = self.stack.last_mut() {
            let top = unsafe { &**cur };
            let cld = match top.nested(*idx) {
                Some(obj) => obj,
                None => {
                    self.pop();
                    continue;
                }
            };

            *idx += 1;

            return cld.handle(self).await;
        }

        self.done()
    }

    pub fn done(mut self) -> FibraResult<Response<Body>> {
        Ok(std::mem::take(&mut self.res))
    }
}
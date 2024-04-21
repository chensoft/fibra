use crate::fibra::*;
use crate::route::*;
use crate::types::*;

// todo default generic type: pub struct HeaderMap<T = HeaderValue>
pub struct Context {
    pub app: Arc<Fibra>, // todo set config in fibra, read from this
    pub req: Request,

    param: IndexMap<String, String>, // todo radix map
    stack: Vec<(*const dyn Handler, usize)>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(app: Arc<Fibra>, req: Request) -> Self {
        Self { app, req, param: IndexMap::new(), stack: vec![] }
    }

    pub fn params(&self) -> &IndexMap<String, String> {
        &self.param
    }

    pub fn param(&self, _key: &str) -> &str {
        todo!()
    }
}

impl Context {
    // todo check cqueues
    pub async fn read(&mut self) {
        todo!()
    }

    pub async fn read_line(&mut self) {
        todo!()
    }

    pub async fn read_until(&mut self) {
        todo!()
    }

    pub async fn save(&mut self, _path: &str) {
        todo!()
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

    pub async fn next(mut self) -> FibraResult<Response> {
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

        Ok(Response::default())
    }

    pub async fn reject(self, status: Option<Status>) -> FibraResult<Response> {
        Ok(Response::default().status(status.unwrap_or(Status::FORBIDDEN)))
    }

    pub async fn rewrite(mut self, to: &'static str, body: Vec<u8>) -> FibraResult<Response> {
        // todo no body
        self.req.parts_mut().uri = Uri::from_static(to); // todo right?
        // self.req.body_mut() = ;

        let app = self.app;
        let ctx = Context::new(app.clone(), self.req);

        app.handle(ctx).await
    }

    pub async fn redirect(self, to: Uri, status: Option<Status>) -> FibraResult<Response> {
        Ok(Response::default()
            .status(status.unwrap_or(Status::TEMPORARY_REDIRECT))
            .header(header::LOCATION, HeaderValue::from_str(to.to_string().as_str())?))
    }
}
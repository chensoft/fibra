use crate::types::*;
use crate::route::*;
use crate::fibra::*;

pub struct Context {
    pub root: Arc<Fibra>,
    pub sock: SocketAddr,
    pub peer: SocketAddr,
    pub head: hyper::http::request::Parts,
    pub body: Body,
    pub lifo: Vec<(*const dyn Handler, usize)>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(root: Arc<Fibra>, sock: SocketAddr, peer: SocketAddr, req: Request<Body>) -> Self {
        let (head, body) = req.into_parts();
        Self { root, sock, peer, head, body, lifo: vec![] }
    }

    pub fn method(&self) -> &Method {
        &self.head.method
    }

    pub fn path(&self) -> &str {
        &self.head.uri.path()
    }

    pub fn query(&self) -> &str {
        &self.head.uri.query().unwrap_or("")
    }

    pub fn headers(&self) -> &header::HeaderMap {
        &self.head.headers
    }

    pub fn param(&self, _key: &str) { todo!() }

    // todo tls info, ver, sni
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
        self.head.uri = Uri::from_static(to);

        let app = self.root.clone();
        let ctx = Context {
            root: app.clone(),
            sock: self.sock,
            peer: self.peer,
            head: std::mem::replace(&mut self.head, Request::<()>::default().into_parts().0),
            body: std::mem::take(&mut self.body),
            lifo: vec![],
        };

        app.handle(ctx).await
    }

    pub async fn redirect(&mut self, to: Uri, status: Option<StatusCode>) -> FibraResult<Response<Body>> {
        Ok(Response::builder()
            .status(status.unwrap_or(StatusCode::TEMPORARY_REDIRECT))
            .header(header::LOCATION, header::HeaderValue::from_str(to.to_string().as_str())?)
            .body(Body::default())?)
    }
}

impl Context {
    #[inline]
    pub fn push(&mut self, cur: *const dyn Handler) {
        self.lifo.push((cur, 0));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.lifo.pop();
    }

    pub async fn next(mut self) -> FibraResult<Response<Body>> {
        while let Some((cur, idx)) = self.lifo.last_mut() {
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
}
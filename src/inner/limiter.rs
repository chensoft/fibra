use crate::inner::*;

#[derive(Default)]
pub struct Limiter {
    pub limits: Vec<Box<dyn Fn(&Context) -> StatusCode + Send + Sync + 'static>>
}

impl Limiter {
    pub fn add(&mut self, limit: impl Fn(&Context) -> StatusCode + Send + Sync + 'static) -> &mut Self {
        self.limits.push(Box::new(limit));
        self
    }

    pub fn pass(&self, ctx: &Context) -> StatusCode {
        self.limits.iter().find_map(|f| {
            let status = f(ctx);
            match status == StatusCode::OK {
                true => None,
                false => Some(status),
            }
        }).unwrap_or(StatusCode::OK)
    }

    pub fn clear(&mut self) -> &mut Self {
        self.limits.clear();
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.add(move |ctx| match ctx.req.method() == method {
            true => StatusCode::OK,
            false => StatusCode::METHOD_NOT_ALLOWED
        });
        self
    }

    // todo use Into Method
    pub fn methods(&mut self) -> &mut Self {
        todo!()
    }

    pub fn scheme(&mut self) -> &mut Self {
        todo!()
    }

    pub fn schemes(&mut self) -> &mut Self {
        todo!()
    }

    pub fn version(&mut self) -> &mut Self {
        todo!()
    }

    pub fn versions(&mut self) -> &mut Self {
        todo!()
    }

    pub fn host(&mut self, _pattern: impl Into<Pattern>) -> &mut Self {
        // todo self.add(move |ctx| ctx.req.uri().host() == Some(value.as_str()));
        self
    }

    pub fn hosts(&mut self) -> &mut Self {
        todo!()
    }

    pub fn port(&mut self) -> &mut Self {
        todo!()
    }
    pub fn ports(&mut self) -> &mut Self {
        todo!()
    }

    pub fn path(&mut self) -> &mut Self {
        todo!()
    }
    pub fn paths(&mut self) -> &mut Self {
        todo!()
    }

    pub fn query(&mut self) -> &mut Self {
        todo!()
    }

    pub fn queries(&mut self) -> &mut Self {
        todo!()
    }

    pub fn header(&mut self, key: header::HeaderName, val: header::HeaderValue) -> &mut Self {
        self.add(move |ctx| match ctx.req.headers().get(&key) == Some(&val) {
            true => StatusCode::OK,
            false => StatusCode::BAD_REQUEST,
        });
        self
    }

    pub fn headers(&mut self) -> &mut Self {
        todo!()
    }
}

#[async_trait]
impl Handler for Limiter {
    async fn handle(&self, mut ctx: Context) -> FibraResult<Response<Body>> {
        if self.pass(&ctx) != StatusCode::OK {
            ctx.pop();
        }

        ctx.next().await
    }
}
use crate::route::*;
use crate::types::*;

#[derive(Default)]
pub struct Limiter {
    pub limits: Vec<Box<dyn Fn(&Context) -> Status + Send + Sync + 'static>>
}

impl Limiter {
    pub fn add(&mut self, limit: impl Fn(&Context) -> Status + Send + Sync + 'static) -> &mut Self {
        self.limits.push(Box::new(limit));
        self
    }

    pub fn pass(&self, ctx: &Context) -> Status {
        self.limits.iter().find_map(|f| {
            let status = f(ctx);
            match status == Status::OK {
                true => None,
                false => Some(status),
            }
        }).unwrap_or(Status::OK)
    }

    pub fn clear(&mut self) -> &mut Self {
        self.limits.clear();
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.add(move |ctx| match ctx.method() == method {
            true => Status::OK,
            false => Status::METHOD_NOT_ALLOWED
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

    pub fn host(&mut self, _pattern: &'static str) -> &mut Self {
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

    pub fn header(&mut self, key: HeaderName, val: HeaderValue) -> &mut Self {
        self.add(move |ctx| match ctx.header(&key) == Some(&val) {
            true => Status::OK,
            false => Status::BAD_REQUEST,
        });
        self
    }

    pub fn headers(&mut self) -> &mut Self {
        todo!()
    }

    pub fn body_size(&mut self) -> &mut Self {
        todo!()
    }
}

#[async_trait]
impl Handler for Limiter {
    async fn handle(&self, mut ctx: Context) -> BoltResult<Response> {
        if self.pass(&ctx) != Status::OK {
            ctx.pop();
        }

        ctx.next().await
    }
}
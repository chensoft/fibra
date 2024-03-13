use crate::inner::*;

#[derive(Default)]
pub struct Limiter {
    pub limits: Vec<Box<dyn Fn(&Context) -> bool + Send + Sync + 'static>>
}

impl Limiter {
    pub fn add(&mut self, limit: impl Fn(&Context) -> bool + Send + Sync + 'static) -> &mut Self {
        self.limits.push(Box::new(limit));
        self
    }

    pub fn pass(&self, ctx: &Context) -> bool {
        self.limits.iter().all(|f| f(ctx))
    }

    pub fn clear(&mut self) -> &mut Self {
        self.limits.clear();
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.add(move |ctx| ctx.req.method() == method);
        self
    }

    pub fn host(&mut self, _pattern: impl Into<Pattern>) -> &mut Self {
        // todo self.add(move |ctx| ctx.req.uri().host() == Some(value.as_str()));
        self
    }

    pub fn header(&mut self, key: header::HeaderName, val: header::HeaderValue) -> &mut Self {
        self.add(move |ctx| ctx.req.headers().get(&key) == Some(&val));
        self
    }
}

#[async_trait]
impl Handler for Limiter {
    async fn handle(&self, mut ctx: Context) -> Result<Response<Body>> {
        if !self.pass(&ctx) {
            ctx.pop();
        }

        ctx.next().await
    }
}
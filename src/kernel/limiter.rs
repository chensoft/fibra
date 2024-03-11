use crate::kernel::*;

#[derive(Default)]
pub struct Limiter {
    pub limits: Vec<Box<dyn Fn(&Context) -> bool + Send + Sync + 'static>>
}

impl Limiter {
    pub fn add(&mut self, limit: impl Fn(&Context) -> bool + Send + Sync + 'static) -> &mut Self {
        self.limits.push(Box::new(limit));
        self
    }

    pub fn pass(&self, _ctx: &Context) -> bool {
        todo!()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.limits.clear();
        self
    }

    pub fn method(&mut self, _method: Method) -> &mut Self {
        todo!()
    }

    pub fn host(&mut self, _pattern: impl Into<Pattern>) -> &mut Self {
        // self.add(move |ctx| ctx.req.uri().host() == Some(value.as_str()));
        todo!()
    }

    pub fn header(&mut self) -> &mut Self {
        todo!()
    }
}

#[async_trait]
impl Handler for Limiter {
    async fn handle(&self, _ctx: Context) -> Result<Response<Body>> {
//         if !self.limits.iter().all(|f| f(ctx)) {
//             ctx.skip();
//         }
// 
//         ctx.next().await
        todo!()
    }
}
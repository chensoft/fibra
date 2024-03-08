use crate::kernel::*;

pub struct Closure {
    pub f: Box<dyn Fn(Context) -> BoxFuture<'static, Result<Response<Body>>> + Send + Sync + 'static>,
}

impl Closure {
    pub fn new(f: impl Fn(Context) -> BoxFuture<'static, Result<Response<Body>>> + Send + Sync + 'static) -> Self {
        Self { f: Box::new(f) }
    }
}

#[async_trait]
impl Handler for Closure {
    async fn handle(&self, ctx: Context) -> Result<Response<Body>> {
        (self.f)(ctx).await
    }
}
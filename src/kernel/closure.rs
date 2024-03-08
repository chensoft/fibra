use crate::kernel::*;

pub struct Closure {
    pub f: Box<dyn for<'a> Fn(&'a mut Context) -> BoxFuture<'a, Result<()>> + Send + Sync + 'static>,
}

impl Closure {
    pub fn new(f: impl for<'a> Fn(&'a mut Context) -> BoxFuture<'a, Result<()>> + Send + Sync + 'static) -> Self {
        Self { f: Box::new(f) }
    }
}

#[async_trait]
impl Handler for Closure {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        (self.f)(ctx).await
    }
}
use crate::consts::*;
use crate::kernel::*;

// todo rename
pub struct Router {
    pub method: Method,
    pub handler: Box<dyn Handler>,
}

impl Router {
    pub fn new(method: Method, handler: impl Handler) -> Self {
        Self {method, handler: Box::new(handler)}
    }
}

#[async_trait]
impl Handler for Router {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        match ctx.req.method() == self.method {
            true => self.handler.handle(ctx).await,
            false => ctx.next().await,
        }
    }
}
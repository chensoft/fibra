use crate::consts::*;
use crate::kernel::*;

// todo rename
pub struct Methods {
    pub method: Method,
    pub handler: Box<dyn Handler>,
}

impl Methods {
    pub fn new(method: Method, handler: impl Handler) -> Self {
        Self { method, handler: Box::new(handler) }
    }
}

#[async_trait]
impl Handler for Methods {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        match ctx.req.method() == self.method {
            true => self.handler.handle(ctx).await,
            false => ctx.next().await,
        }
    }
}

// match self.Method.get(ctx.search.as_str()) {
//     Some(val) => ctx.routes.push_front(val),
//     None => return Err(StatusCode::NOT_FOUND.into_error()),
// };

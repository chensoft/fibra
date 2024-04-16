use crate::types::*;
use crate::route::*;

pub struct Routine {
    limiter: Limiter,
    handler: BoxHandler,
}

impl Routine {
    pub fn new(handler: impl Handler) -> Self {
        Self { limiter: Limiter::default(), handler: Box::new(handler) }
    }

    pub fn limit(&mut self) -> &mut Limiter {
        &mut self.limiter
    }

    pub fn trust<T: Handler>(&mut self) -> &mut T {
        self.handler.as_handler_mut::<T>().unwrap_or_else(|| unreachable!())
    }
}

#[async_trait]
impl Handler for Routine {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        let status = self.limiter.pass(&ctx);
        match status == StatusCode::OK {
            true => self.handler.handle(ctx).await,
            false => Err(status.into_error())
        }
    }
}
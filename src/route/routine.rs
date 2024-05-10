use crate::route::*;
use crate::types::*;

pub struct Routine {
    limiter: Limiter,
    handler: BoxHandler,
}

impl Routine {
    pub fn from(handler: impl Handler) -> Self {
        Self { limiter: Limiter::default(), handler: Box::new(handler) }
    }

    pub fn limit(&mut self) -> &mut Limiter {
        &mut self.limiter
    }

    pub fn treat<T: Handler>(&mut self) -> Option<&mut T> {
        self.handler.as_handler_mut::<T>()
    }
}

#[async_trait]
impl Handler for Routine {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        todo!()
    }
}
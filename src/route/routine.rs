//! Store HTTP route info
use crate::{FibraResult, Response};
use crate::route::*;

/// A struct that stores HTTP route information
pub struct Routine {
    /// Filters will run before execute handler
    limiter: Option<Limiter>,

    /// The http handler
    handler: BoxHandler,
}

impl Routine {
    /// Create a new object
    #[inline]
    pub fn from(handler: impl Handler) -> Self {
        Self { limiter: None, handler: Box::new(handler) }
    }

    /// Get the limiter
    #[inline]
    pub fn limit(&mut self) -> &mut Limiter {
        self.limiter.get_or_insert(Limiter::new())
    }

    /// Treat the handler as type T
    #[inline]
    pub fn treat<T: Handler>(&mut self) -> Option<&mut T> {
        self.handler.as_handler_mut::<T>()
    }
}

#[async_trait]
impl Handler for Routine {
    #[inline]
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        if let Some(limiter) = &self.limiter {
            if !limiter.test(&ctx) {
                return ctx.next().await;
            }
        }

        self.handler.handle(ctx).await
    }
}
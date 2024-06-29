//! Store HTTP route info
use crate::{FibraResult, Response};
use crate::route::*;

/// A struct that stores HTTP route information
pub struct Routine {
    /// Filters will run before execute service
    limiter: Option<Limiter>,

    /// The http service
    service: BoxService,
}

impl Routine {
    /// Create a new object
    #[inline]
    pub fn from(service: impl Service) -> Self {
        Self { limiter: None, service: Box::new(service) }
    }

    /// Get the limiter
    #[inline]
    pub fn limit(&mut self) -> &mut Limiter {
        self.limiter.get_or_insert(Limiter::new())
    }

    /// Treat the service as type T
    #[inline]
    pub fn treat<T: Service>(&mut self) -> Option<&mut T> {
        self.service.as_service_mut::<T>()
    }
}

#[async_trait]
impl Service for Routine {
    #[inline]
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        if let Some(limiter) = &self.limiter {
            if !limiter.test(&ctx) {
                return ctx.next().await;
            }
        }

        self.service.handle(ctx).await
    }
}
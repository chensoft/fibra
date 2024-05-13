//! Store HTTP route info
use crate::route::*;

/// A struct that stores HTTP route information
pub struct Routine {
    /// Filters will run before execute handler
    limiter: Limiter,

    /// The http handler
    handler: BoxHandler,
}

impl Routine {
    /// Create a new routine
    pub fn from(handler: impl Handler) -> Self {
        Self { limiter: Limiter::default(), handler: Box::new(handler) }
    }

    /// Get the limiter
    pub fn limit(&mut self) -> &mut Limiter {
        &mut self.limiter
    }

    /// Treat the handler as type T
    pub fn treat<T: Handler>(&mut self) -> Option<&mut T> {
        self.handler.as_handler_mut::<T>()
    }
}
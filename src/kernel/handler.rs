use crate::consts::*;
use crate::kernel::*;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: Sync + Send + 'static {
    async fn handle(&self, ctx: Context) -> Result<Context>;
}

/// To store arbitrary handlers
pub trait AnyHandler: Handler + Any {
    /// Treat object as Any
    fn as_any(&self) -> &dyn Any;

    /// Treat object as Any
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Handler + Any> AnyHandler for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[async_trait]
impl<F, R> Handler for F
    where F: Fn(Context) -> R + Sync + Send + 'static,
          R: Future<Output = Result<Context>> + Sync + Send + 'static,
{
    async fn handle(&self, ctx: Context) -> Result<Context> {
        self(ctx).await
    }
}
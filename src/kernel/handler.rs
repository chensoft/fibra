use crate::consts::*;
use crate::kernel::*;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: Any + Sync + Send + 'static {
    async fn handle(&self, ctx: Context) -> Result<()>;
}

#[async_trait]
impl<F, R> Handler for F
    where F: Fn(Context) -> R + Sync + Send + 'static,
          R: Future<Output = Result<()>> + Sync + Send + 'static,
{
    async fn handle(&self, ctx: Context) -> Result<()> {
        self(ctx).await
    }
}
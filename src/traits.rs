use crate::consts::Result;
use crate::kernel::Context;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: Sync + Send + 'static {
    async fn handle(&self, ctx: Context) -> Result<()>;
}
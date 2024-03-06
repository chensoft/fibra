use crate::kernel::*;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: Any + Send + Sync + 'static {
    async fn handle(&self, ctx: &mut Context) -> Result<()>;
}
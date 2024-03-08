use crate::kernel::*;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: Any + Send + Sync + 'static {
    async fn next(&self, ctx: Context) -> Result<()> {
        Ok(())
    }

    async fn call(&self, ctx: Context) -> Result<()>;
}
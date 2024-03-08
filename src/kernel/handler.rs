use crate::kernel::*;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: Any + Send + Sync + 'static {
    async fn warmup(&mut self) -> Result<()> {
        Ok(())
    }

    async fn handle(&self, ctx: Context) -> Result<Response<Body>>;
}
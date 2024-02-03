use super::general::*;
use super::context::*;

#[async_trait]
pub trait Handler: Sync + Send + 'static {
    async fn handle(&mut self, ctx: Context) -> Result<()>;
}
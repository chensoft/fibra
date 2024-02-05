#[async_trait]
pub trait Handler: Sync + Send + 'static {
    async fn handle(&self, ctx: Context) -> Result<()>;
}
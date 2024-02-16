use crate::consts::*;
use crate::kernel::*;

pub struct Logger;

#[async_trait]
impl Handler for Logger {
    async fn handle(&self, ctx: Context) -> Result<()> {
        // todo
        logkit::warn!("{} {} {}", ctx.req.uri().scheme_str().unwrap_or(&"unknown"), ctx.req.method(), ctx.req.uri().path());
        Ok(())
    }
}
use crate::consts::*;
use crate::kernel::*;

pub struct Timeout;

#[async_trait]
impl Handler for Timeout {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        todo!()
    }
}
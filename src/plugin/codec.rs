use crate::consts::*;
use crate::kernel::*;

pub struct Codec;

#[async_trait]
impl Handler for Codec {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        todo!()
    }
}
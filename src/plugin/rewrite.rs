use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Rewrite;

#[async_trait]
impl Handler for Rewrite {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Reject;

#[async_trait]
impl Handler for Reject {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
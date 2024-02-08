use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Recover;

#[async_trait]
impl Handler for Recover {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
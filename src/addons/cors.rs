use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Cors;

#[async_trait]
impl Handler for Cors {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
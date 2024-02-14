use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Limiter;

#[async_trait]
impl Handler for Limiter {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
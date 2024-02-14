use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct RealIP;

#[async_trait]
impl Handler for RealIP {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
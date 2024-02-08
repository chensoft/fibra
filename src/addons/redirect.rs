use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Redirect;

#[async_trait]
impl Handler for Redirect {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
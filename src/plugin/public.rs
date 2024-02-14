use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Public;

#[async_trait]
impl Handler for Public {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
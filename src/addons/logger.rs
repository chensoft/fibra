use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Logger;

#[async_trait]
impl Handler for Logger {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
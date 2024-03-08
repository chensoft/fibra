use crate::kernel::*;

/// Remove duplicate slashes
pub struct Normalize;

#[async_trait]
impl Handler for Normalize {
    async fn handle(&self, _ctx: Context) -> Result<()> {
        todo!()
    }
}
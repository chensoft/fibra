use crate::types::*;
use crate::route::*;

/// Remove duplicate slashes
pub struct Normalize;

#[async_trait]
impl Handler for Normalize {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response<Body>> {
        todo!()
    }
}
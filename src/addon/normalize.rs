use crate::types::*;
use crate::inner::*;

/// Remove duplicate slashes
/// todo https://url.spec.whatwg.org/#example-url-parsing
pub struct Normalize;

#[async_trait]
impl Handler for Normalize {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response<Body>> {
        todo!()
    }
}
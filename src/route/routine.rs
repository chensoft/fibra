use crate::route::*;
use crate::types::*;

pub struct Routine {
}

impl Routine {
}

impl Default for Routine {
    fn default() -> Self {
        todo!()
    }
}

#[async_trait]
impl Handler for Routine {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        todo!()
    }
}
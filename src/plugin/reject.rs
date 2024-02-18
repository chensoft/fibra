use crate::consts::*;
use crate::kernel::*;

pub struct Reject {
    pub status: Option<http::StatusCode>,
}

impl Reject {
    pub fn new(status: Option<http::StatusCode>) -> Self {
        Self {status}
    }
}

#[async_trait]
impl Handler for Reject {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        ctx.reject(self.status.clone())
    }
}
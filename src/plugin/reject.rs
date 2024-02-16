use crate::consts::*;
use crate::kernel::*;

pub struct Reject {
    pub status: http::StatusCode,
}

impl Reject {
    pub fn new(status: Option<http::StatusCode>) -> Self {
        Self {status: status.unwrap_or(http::StatusCode::FORBIDDEN)}
    }
}

#[async_trait]
impl Handler for Reject {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
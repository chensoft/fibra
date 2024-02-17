use crate::consts::*;
use crate::kernel::*;

pub struct Redirect {
    pub to: http::Uri,
    pub status: http::StatusCode,
}

impl Redirect {
    pub fn new(to: impl Into<http::Uri>, status: Option<http::StatusCode>) -> Self {
        Self {to: to.into(), status: status.unwrap_or(http::StatusCode::TEMPORARY_REDIRECT)}
    }
}

#[async_trait]
impl Handler for Redirect {
    async fn handle(&self, _ctx: Context) -> Result<Context> {
        todo!()
    }
}
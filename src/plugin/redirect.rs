use crate::consts::*;
use crate::kernel::*;

pub struct Redirect {
    pub to: http::Uri,
    pub status: Option<http::StatusCode>,
}

impl Redirect {
    pub fn new(to: http::Uri, status: Option<http::StatusCode>) -> Self {
        Self {to, status}
    }
}

#[async_trait]
impl Handler for Redirect {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        ctx.redirect(self.to.clone(), self.status.clone())
    }
}
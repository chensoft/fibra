use crate::consts::*;
use crate::kernel::*;

// todo universe Derive
pub struct Rewrite {
    pub to: http::Uri,
}

impl Rewrite {
    pub fn new(to: http::Uri) -> Self {
        Self {to}
    }
}

#[async_trait]
impl Handler for Rewrite {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        ctx.rewrite(self.to.clone()).await
    }
}
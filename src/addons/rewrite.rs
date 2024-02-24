use crate::consts::*;
use crate::kernel::*;

// todo universe Derive
pub struct Rewrite {
    pub to: Uri,
}

impl Rewrite {
    pub fn new(to: Uri) -> Self {
        Self {to}
    }
}

#[async_trait]
impl Handler for Rewrite {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        ctx.rewrite(self.to.clone()).await
    }
}
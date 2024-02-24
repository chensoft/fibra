use crate::consts::*;
use crate::kernel::*;

pub struct Redirect {
    pub to: Uri,
    pub status: Option<StatusCode>,
}

impl Redirect {
    pub fn new(to: Uri, status: Option<StatusCode>) -> Self {
        Self {to, status}
    }
}

#[async_trait]
impl Handler for Redirect {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        ctx.redirect(self.to.clone(), self.status.clone())
    }
}
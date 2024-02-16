use crate::consts::*;
use crate::kernel::*;

// todo universe Derive
pub struct Rewrite {
    pub to: http::Uri,
}

impl Rewrite {
    pub fn new(to: impl Into<http::Uri>) -> Self {
        Self {to: to.into()}
    }
}

#[async_trait]
impl Handler for Rewrite {
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
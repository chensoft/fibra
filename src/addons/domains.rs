use crate::consts::*;
use crate::kernel::*;

pub struct Domain {
    pub domain: Pattern,
    pub handler: Box<dyn Handler>,
}

impl Domain {
    pub fn new(domain: impl Into<Pattern>, handler: impl Handler) -> Self {
        Self { domain: domain.into(), handler: Box::new(handler) }
    }
}

#[async_trait]
impl Handler for Domain {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        match self.domain.matches(ctx.req.uri().host().unwrap_or(&"")) {
            true => self.handler.handle(ctx).await,
            false => ctx.next().await,
        }
    }
}
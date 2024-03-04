use crate::consts::*;
use crate::kernel::*;

#[derive(Default)]
pub struct Domains {
    pub domains: Vec<Pattern>, // todo use preway
}

impl Domains {
    // todo accept array ["api.*"]
    pub fn new(domains: Vec<impl Into<Pattern>>) -> Self {
        Self { domains: domains.into_iter().map(|d| d.into()).collect() }
    }

    pub fn add(&mut self, domain: impl Into<Pattern>) {
        self.domains.push(domain.into());
    }
}

#[async_trait]
impl Handler for Domains {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        // match self.domain.matches(ctx.req.uri().host().unwrap_or(&"")) {
        //     true => self.handler.handle(ctx).await,
        //     false => ctx.next().await,
        // }
        todo!()
    }
}
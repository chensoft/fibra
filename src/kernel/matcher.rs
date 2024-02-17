use crate::consts::*;
use crate::kernel::*;

pub struct Matcher {
    pub preway: IndexMap<Pattern, Box<dyn Handler>> // todo multiple handler
}

impl Matcher {
    pub fn new() -> Self {
        Self {preway: IndexMap::new()}
    }

    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        let pattern = pattern.into();
        match self.preway.get_mut(&pattern) {
            None => { self.preway.insert(pattern, Box::new(handler)); }
            Some(val) => *val = Box::new(handler),
        }
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        if let Some(val) = self.preway.get(&Pattern::Plain(ctx.req.uri().path().to_string())) {
            return val.handle(ctx).await;
        }

        ctx.next().await
    }
}
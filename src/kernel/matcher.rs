use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Matcher {
    pub preway: IndexMap<Pattern, Box<dyn Handler>>
}

impl Default for Matcher {
    fn default() -> Self {
        Self {preway: IndexMap::new()}
    }
}

impl Matcher {
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
    async fn handle(&self, mut ctx: Context) -> Result<()> {
        match self.preway.get(&ctx.req.uri().path()) {
            None => ctx.next().await,
            Some(val) => val.handle(ctx).await,
        }
    }
}
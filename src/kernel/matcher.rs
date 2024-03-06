use crate::kernel::*;

#[derive(Default)]
pub struct Matcher {
    pub limiter: Limiter,
    pub mapping: HashMap<Pattern, Box<dyn Handler>> // todo multiple handler
}

impl Matcher {
    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        // for each Method add all except some special methods
        todo!()
    }

    pub fn get(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        todo!()
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn warmup(&mut self) -> Result<()> {
        todo!()
    }

    async fn handle(&self, _ctx: &mut Context) -> Result<()> {
        todo!()
    }
}
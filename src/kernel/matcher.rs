use crate::{Handler, Context, Pattern};

#[derive(Debug, Clone)]
pub struct Matcher;

impl Matcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, ctx: Context) -> crate::Result<()> {
        todo!()
    }
}
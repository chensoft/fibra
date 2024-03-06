use crate::kernel::*;

// todo add closure type alias to gen func
#[derive(Default)]
pub struct Limiter {
    pub limits: Vec<Box<dyn Fn(&Context) -> bool + Send + Sync + 'static>>
}

impl Limiter {
    pub fn add(mut self, limit: impl Fn(&Context) -> bool + Send + Sync + 'static) -> Self {
        self.limits.push(Box::new(limit));
        self
    }
}

#[async_trait]
impl Handler for Limiter {
    async fn handle(&self, _ctx: &mut Context) -> Result<()> {
        todo!()
    }
}
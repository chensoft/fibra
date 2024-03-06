use crate::kernel::*;
use crate::limits::*;

#[derive(Default)]
pub struct Routine {
    pub limiter: Limiter,
    pub handler: Option<Box<dyn Handler>>,
    pub finally: Arc<Vec<Box<dyn Handler>>>,
}

impl Routine {
    pub fn any(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Limiter {
        self.handler = Some(Box::new(handler));
        self.limiter.get().post(); // todo more
        self.limiter.path(pattern.into());
        &mut self.limiter
    }

    pub fn get(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Limiter {
        self.handler = Some(Box::new(handler));
        self.limiter.get().path(pattern.into());
        &mut self.limiter
    }
}

#[async_trait]
impl Handler for Routine {
    async fn warmup(&mut self) -> Result<()> {
        if let Some(handler) = self.handler.take() {
            self.finally = Arc::new(vec![handler]);
        }

        Ok(())
    }

    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        ctx.push(self.finally.clone(), 0);
        self.limiter.handle(ctx).await
    }
}
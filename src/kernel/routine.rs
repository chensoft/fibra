use crate::kernel::*;

pub struct Routine {
    limiter: Limiter,
    handler: Option<Box<dyn Handler>>,
    finally: Arc<Vec<Box<dyn Handler>>>,
}

impl Routine {
    pub fn new(handler: impl Handler) -> Self {
        Self { limiter: Limiter::default(), handler: Some(Box::new(handler)), finally: Arc::new(vec![]) }
    }

    pub fn limit(&mut self) -> &mut Limiter {
        &mut self.limiter
    }

    pub fn treat<T: Handler>(&mut self) -> &mut T {
        match &mut self.handler {
            Some(obj) => match obj.as_mut().as_any_mut().downcast_mut::<T>() {
                Some(obj) => obj,
                None => unreachable!()
            }
            None => unreachable!()
        }
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

    async fn handle(&self, mut ctx: Context) -> Result<Response<Body>> {
        if self.limiter.ok(&ctx) {
            ctx.push(self.finally.clone(), 0);
        }

        ctx.next().await
    }
}
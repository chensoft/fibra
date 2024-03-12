use crate::kernel::*;

pub struct Routine {
    limiter: Limiter,
    handler: Package,
}

impl Routine {
    pub fn new(handler: impl Handler) -> Self {
        Self { limiter: Limiter::default(), handler: Package::new(vec![handler]) }
    }

    pub fn limit(&mut self) -> &mut Limiter {
        &mut self.limiter
    }

    pub fn trust<T: Handler>(&mut self) -> &mut T {
        match self.handler.iter_mut::<T>().next() {
            Some(obj) => obj,
            _ => unreachable!()
        }
    }
}

#[async_trait]
impl Handler for Routine {
    async fn warmup(&mut self) -> Result<()> {
        self.handler.warmup().await
    }

    async fn handle(&self, ctx: Context) -> Result<Response<Body>> {
        match self.limiter.pass(&ctx) {
            true => self.handler.handle(ctx).await,
            false => ctx.next().await
        }
    }
}
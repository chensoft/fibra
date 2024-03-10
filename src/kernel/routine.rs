use crate::kernel::*;
// use crate::limits::*;

#[derive(Default)]
pub struct Routine {
    limiter: Limiter,
    handler: Option<Box<dyn Handler>>,
    finally: Arc<Vec<Box<dyn Handler>>>,
}

impl Routine {
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

    pub fn all<T: Handler>(&mut self, handler: T) -> &mut Self {
        self.handler = Some(Box::new(handler));
        self.limiter
            .clear()
            .method(Method::GET)
            .method(Method::POST)
            .method(Method::PUT)
            .method(Method::DELETE)
            .method(Method::HEAD)
            .method(Method::OPTIONS)
            .method(Method::CONNECT)
            .method(Method::PATCH)
            .method(Method::TRACE)
            .method(Method::PATCH);
        self
    }

    pub fn get<T: Handler>(&mut self, handler: T) -> &mut Self {
        self.handler = Some(Box::new(handler));
        self.limiter.clear().method(Method::GET);
        self
    }

    pub fn post<T: Handler>(&mut self, handler: T) -> &mut Self {
        self.handler = Some(Box::new(handler));
        self.limiter.clear().method(Method::POST);
        self
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
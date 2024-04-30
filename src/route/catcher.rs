use crate::route::*;
use crate::types::*;

pub struct Catcher {
    pub default: Box<dyn Fn(BoltError) -> Response + Send + Sync + 'static>,
    pub handler: Box<dyn Fn(&Catcher, BoltError) -> Response + Send + Sync + 'static>,
}

impl Catcher {
    pub fn new(f: impl Fn(&Catcher, BoltError) -> Response + Send + Sync + 'static) -> Self {
        Self { handler: Box::new(f), ..Default::default() }
    }
}

impl Default for Catcher {
    fn default() -> Self {
        let default = Box::new(|_| {
            Response::default().status(Status::INTERNAL_SERVER_ERROR)
        });

        Self { default, handler: Box::new(|obj, err| (obj.default)(err)) }
    }
}

#[async_trait]
impl Handler for Catcher {
    async fn handle(&self, ctx: Context) -> BoltResult<Response> {
        use futures::FutureExt;

        match AssertUnwindSafe(ctx.next()).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(res) => Ok(res),
                Err(err) => Ok((self.handler)(self, err)),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => Ok((self.handler)(self, BoltError::PanicError(err.to_string()))),
                None => Ok((self.handler)(self, BoltError::PanicError("Unknown panic".to_string()))),
            }
        }
    }
}
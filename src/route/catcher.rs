use crate::types::*;
use crate::route::*;

pub struct Catcher {
    pub default: Box<dyn Fn(FibraError) -> Response<Body> + Send + Sync + 'static>,
    pub handler: Box<dyn Fn(&Catcher, FibraError) -> Response<Body> + Send + Sync + 'static>,
}

impl Catcher {
    pub fn new(f: impl Fn(&Catcher, FibraError) -> Response<Body> + Send + Sync + 'static) -> Self {
        Self { handler: Box::new(f), ..Default::default() }
    }
}

impl Default for Catcher {
    fn default() -> Self {
        let default = Box::new(|err: FibraError| {
            match err {
                FibraError::StatusCode(status) => status.into_response(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        });

        Self { default, handler: Box::new(|obj, err| (obj.default)(err)) }
    }
}

#[async_trait]
impl Handler for Catcher {
    async fn handle(&self, ctx: Context) -> FibraResult<Response<Body>> {
        use futures::FutureExt;

        match AssertUnwindSafe(ctx.next()).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(res) => Ok(res),
                Err(err) => Ok((self.handler)(self, err)),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => Ok((self.handler)(self, FibraError::PanicError(err.to_string()))),
                None => Ok((self.handler)(self, FibraError::PanicError("Unknown panic".to_string()))),
            }
        }
    }
}
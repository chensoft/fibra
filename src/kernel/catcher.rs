use crate::kernel::*;

pub struct Catcher {
    pub handler: Box<dyn Fn(anyhow::Error) -> Response<Body> + Send + Sync + 'static>,
    pub default: Box<dyn Fn(anyhow::Error) -> Response<Body> + Send + Sync + 'static>,
}

impl Default for Catcher {
    fn default() -> Self {
        let default = Box::new(|err: anyhow::Error| {
            match err.downcast_ref::<Error>() {
                Some(Error::StatusCode(status)) => status.into_response(),
                None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        });

        Self { handler: default.clone(), default }
    }
}

impl Catcher {
    pub fn new(f: impl Fn(anyhow::Error) -> Response<Body> + Send + Sync + 'static) -> Self {
        Self { handler: Box::new(f), ..Default::default() }
    }
}

#[async_trait]
impl Handler for Catcher {
    async fn handle(&self, ctx: Context) -> Result<Response<Body>> {
        use futures::FutureExt;

        match AssertUnwindSafe(ctx.next()).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(res) => Ok(res),
                Err(err) => Ok((self.handler)(err)),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => Ok((self.handler)(anyhow!(err.to_string()))),
                None => Ok((self.handler)(anyhow!("Unknown panic"))),
            }
        }
    }
}
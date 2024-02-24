use crate::consts::*;
use crate::kernel::*;

pub struct Catcher {
    pub catch: Box<dyn Fn(&mut Context, anyhow::Error) + Send + Sync + 'static>,
}

impl Catcher {
    /// ```
    /// use veloce::*;
    /// 
    /// addons::Catcher::new(|ctx, err| {
    ///     match err.downcast_ref::<Error>() {
    ///         Some(Error::HttpStatusCode(status)) => ctx.res = status.into_response(),
    ///         _ => ctx.res = StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    ///     }
    /// });
    /// ```
    pub fn new(catch: impl Fn(&mut Context, anyhow::Error) + Send + Sync + 'static) -> Self {
        Self {catch: Box::new(catch)}
    }
}

#[async_trait]
impl Handler for Catcher {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        use futures::FutureExt;

        match AssertUnwindSafe(ctx.next()).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(_) => {}
                Err(err) => (self.catch)(ctx, err),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => (self.catch)(ctx, anyhow!(err.to_string())),
                None => (self.catch)(ctx, anyhow!("Unknown panic")),
            }
        };

        Ok(())
    }
}
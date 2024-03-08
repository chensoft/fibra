use crate::kernel::*;

// todo to async fn
pub struct Catcher {
    pub handler: Box<dyn Fn(&mut Context, anyhow::Error) + Send + Sync + 'static>,
    pub default: Box<dyn Fn(&mut Context, anyhow::Error) + Send + Sync + 'static>,
}

impl Catcher {
    pub fn new(catch: impl Fn(&mut Context, anyhow::Error) + Send + Sync + 'static) -> Self {
        Self {handler: Box::new(catch), ..Default::default()}
    }
}

impl Default for Catcher {
    fn default() -> Self {
        // let default = Box::new(|mut ctx: Context, err: anyhow::Error| {
        //     let res = match err.downcast_ref::<Error>() {
        //         Some(Error::StatusCode(status)) => status.into_response(),
        //         None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        //     };
        //     ctx.res = res;
        // });
        // 
        // Self { handler: default.clone(), default }
        todo!()
    }
}

#[async_trait]
impl Handler for Catcher {
    async fn handle(&self, mut ctx: Context) -> Result<()> {
        // use futures::FutureExt;
        // 
        // match AssertUnwindSafe(ctx.next()).catch_unwind().await {
        //     Ok(ret) => match ret {
        //         Ok(_) => {}
        //         Err(err) => (self.handler)(ctx, err),
        //     }
        //     Err(err) => match err.downcast_ref::<&str>() {
        //         Some(err) => (self.handler)(ctx, anyhow!(err.to_string())),
        //         None => (self.handler)(ctx, anyhow!("Unknown panic")),
        //     }
        // };
        // 
        // Ok(())
        todo!()
    }
}
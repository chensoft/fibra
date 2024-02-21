use crate::consts::*;
use crate::kernel::*;
// use futures::FutureExt;

pub struct Catcher {
    pub catch: fn(anyhow::Error) -> Response<Body>,
}

impl Catcher {
    pub fn new(catch: fn(anyhow::Error) -> Response<Body>) -> Self {
        Self {catch}
    }
}

#[async_trait]
impl Handler for Catcher {
    async fn handle(&self, _ctx: Context) -> Result<Context> {
        // todo how to return ctx???
        todo!()
        // let res = match AssertUnwindSafe(ctx.next()).catch_unwind().await {
        //     Ok(ret) => match ret {
        //         Ok(ctx) => ctx.res,
        //         Err(err) => (self.catch)(err),
        //     }
        //     Err(err) => match err.downcast_ref::<&str>() {
        //         Some(err) => (self.catch)(anyhow!(err)),
        //         None => (self.catch)(anyhow!("Unknown panic")),
        //     }
        // };
    }
}
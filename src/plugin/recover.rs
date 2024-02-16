use crate::consts::*;
use crate::kernel::*;

pub struct Recover {
    // pub error: fn(Context, anyhow::Error) -> http::Response<http::Body>,
}

// let mut resp = http::Response::default();
// 
// *resp.status_mut() = match err.downcast::<Error>() {
// Ok(Error::NotFound(_)) => http::StatusCode::NOT_FOUND,
// _ => http::StatusCode::SERVICE_UNAVAILABLE,
// };
// 
// resp
// match AssertUnwindSafe(context.next()).catch_unwind().await {
// Ok(ret) => match ret {
// Ok(ctx) => Ok::<_, Infallible>(ctx.res),
// Err(err) => Ok::<_, Infallible>((appself.config.error)(context, err)),
// }
// Err(err) => {
// if let Some(err) = err.downcast_ref::<&str>() {
// Ok::<_, Infallible>((appself.config.error)(context, anyhow!(err.to_string())))
// } else if let Some(err) = err.downcast_ref::<String>() {
// Ok::<_, Infallible>((appself.config.error)(context, anyhow!(err.clone())))
// } else {
// Ok::<_, Infallible>((appself.config.error)(context, anyhow!("unknown error")))
// }
// }
// }

#[async_trait]
impl Handler for Recover {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        ctx.next().await
    }
}
use crate::kernel::*;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: Any + Send + Sync + 'static {
    #[allow(unused_variables)]
    fn nested(&self, idx: usize) -> Option<&BoxHandler> {
        None
    }

    async fn handle(&self, ctx: Context) -> Result<Response<Body>>;
}

pub type BoxHandler = Box<dyn Handler>;

// todo give example like `move |ctx| { MOVE async { Ok(Response) } }`
#[async_trait]
impl<F, R> Handler for F
    where
        F: Fn(Context) -> R + Send + Sync + 'static,
        R: Future<Output = Result<Response<Body>>> + Send + 'static
{
    async fn handle(&self, ctx: Context) -> Result<Response<Body>> {
        self(ctx).await
    }
}
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

/// Box Handler
pub type BoxHandler = Box<dyn Handler>;

/// As Handler
pub trait AsHandler<'a> {
    fn as_handler<T: Handler>(&'a self) -> Option<&'a T>;

    fn as_handler_mut<T: Handler>(&'a mut self) -> Option<&'a mut T>;
}

impl<'a> AsHandler<'a> for BoxHandler {
    fn as_handler<T: Handler>(&'a self) -> Option<&'a T> {
        self.as_ref().as_any().downcast_ref::<T>()
    }

    fn as_handler_mut<T: Handler>(&'a mut self) -> Option<&'a mut T> {
        self.as_mut().as_any_mut().downcast_mut::<T>()
    }
}

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
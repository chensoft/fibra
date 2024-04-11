use crate::inner::*;

// todo Handler<T>, T is body's custom type
#[async_trait]
pub trait Handler: AnyHandler + Send + Sync + 'static {
    #[allow(unused_variables)]
    fn nested(&self, idx: usize) -> Option<&BoxHandler> {
        None
    }

    async fn handle(&self, ctx: Context) -> FibraResult<Response<Body>>;
}

/// Any support
pub trait AnyHandler: Any {
    /// Treat object as any
    fn as_any(&self) -> &dyn Any;

    /// Treat object as any mut
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AnyHandler for T {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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
        R: Future<Output = FibraResult<Response<Body>>> + Send + 'static
{
    async fn handle(&self, ctx: Context) -> FibraResult<Response<Body>> {
        self(ctx).await
    }
}
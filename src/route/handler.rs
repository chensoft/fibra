//! Handler Trait
use crate::route::*;
use crate::types::*;

/// The HTTP Handler trait, handler must implement this to process http requests
#[async_trait]
pub trait Handler: AnyHandler + Send + Sync + 'static {
    /// Impl this function to handle http requests, a context contains a connection object and
    /// a current request object, multiple requests may reside on one connection, and these
    /// requests will be handled one by one on different context objects
    async fn handle(&self, ctx: Context) -> BoltResult<Response>;

    /// Internal method to get the child handler of its parent
    #[allow(unused_variables)]
    fn select(&self, idx: usize) -> Option<&BoxHandler> {
        None
    }
}

/// Box Handler
pub type BoxHandler = Box<dyn Handler>;

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

/// As Handler
///
/// # Examples
///
/// ```
/// use async_trait::async_trait;
/// use bolt::{Handler, AsHandler, Context, Response, BoltResult};
///
/// struct HandlerA;
/// struct HandlerB;
///
/// #[async_trait]
/// impl Handler for HandlerA { async fn handle(&self, _ctx: Context) -> BoltResult<Response> { unimplemented!() } }
/// impl Handler for HandlerB { async fn handle(&self, _ctx: Context) -> BoltResult<Response> { unimplemented!() } }
///
/// let mut handler_a: Box<dyn Handler> = Box::new(HandlerA);
/// let mut handler_b: Box<dyn Handler> = Box::new(HandlerB);
///
/// assert_eq!(handler_a.as_handler::<HandlerA>().is_some(), true);
/// assert_eq!(handler_a.as_handler::<HandlerB>().is_some(), false);
/// assert_eq!(handler_b.as_handler::<HandlerA>().is_some(), false);
/// assert_eq!(handler_b.as_handler::<HandlerB>().is_some(), true);
///
/// assert_eq!(handler_a.as_handler_mut::<HandlerA>().is_some(), true);
/// assert_eq!(handler_a.as_handler_mut::<HandlerB>().is_some(), false);
/// assert_eq!(handler_b.as_handler_mut::<HandlerA>().is_some(), false);
/// assert_eq!(handler_b.as_handler_mut::<HandlerB>().is_some(), true);
/// ```
pub trait AsHandler<'a> {
    /// Convert handler if possible
    fn as_handler<T: Handler>(&'a self) -> Option<&'a T>;

    /// Convert handler if possible
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

/// Impl Handler for functions
///
/// # Examples
///
/// ```
/// use bolt::{Handler, Context, Response, BoltResult};
///
/// async fn free_function(_ctx: Context) -> BoltResult<Response> {
///     Ok("It Works!".into())
/// }
///
/// let handler_closure: dyn Handler = |_ctx: Context| async { b"It Works!".into() };
/// let handler_function: dyn Handler = free_function;
///
/// assert_eq!(handler_closure(), Ok(Response::from("It Works!")));
/// assert_eq!(handler_function(), Ok(Response::from("It Works!")));
/// ```
#[async_trait]
impl<F, R> Handler for F
    where
        F: Fn(Context) -> R + Send + Sync + 'static,
        R: Future<Output = BoltResult<Response>> + Send + 'static
{
    async fn handle(&self, ctx: Context) -> BoltResult<Response> {
        self(ctx).await
    }
}
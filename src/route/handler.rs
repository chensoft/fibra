//! HTTP Handler
use crate::route::*;
use crate::types::*;

/// The HTTP Handler trait, handler must implement this to process http requests
#[async_trait]
pub trait Handler: AnyHandler + Send + Sync + 'static {
    /// Impl this function to handle http requests, a context contains a connection object and
    /// a current request object, multiple requests may reside on one connection, and these
    /// requests will be handled one by one on different context objects
    async fn handle(&self, ctx: Context) -> FibraResult<Response>;
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

#[async_trait]
impl Handler for BoxHandler {
    async fn handle<'a>(&'a self, ctx: Context) -> FibraResult<Response> {
        self.as_ref().handle(ctx).await
    }
}

/// As Handler
///
/// # Examples
///
/// ```
/// use fibra::*;
/// use async_trait::*;
///
/// struct HandlerA;
/// struct HandlerB;
///
/// #[async_trait]
/// impl Handler for HandlerA { async fn handle(&self, _ctx: Context) -> FibraResult<Response> { unimplemented!() } }
/// #[async_trait]
/// impl Handler for HandlerB { async fn handle(&self, _ctx: Context) -> FibraResult<Response> { unimplemented!() } }
///
/// let mut handler_a: BoxHandler = Box::new(HandlerA);
/// let mut handler_b: BoxHandler = Box::new(HandlerB);
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
pub trait AsHandler {
    /// Convert handler if possible
    fn as_handler<T: Handler>(&self) -> Option<&T>;

    /// Convert handler if possible
    fn as_handler_mut<T: Handler>(&mut self) -> Option<&mut T>;
}

impl AsHandler for BoxHandler {
    fn as_handler<T: Handler>(&self) -> Option<&T> {
        self.as_ref().as_any().downcast_ref::<T>()
    }

    fn as_handler_mut<T: Handler>(&mut self) -> Option<&mut T> {
        self.as_mut().as_any_mut().downcast_mut::<T>()
    }
}

/// Impl Handler for vector
#[async_trait]
impl<T: Handler> Handler for Vec<T> {
    async fn handle(&self, mut ctx: Context) -> FibraResult<Response> {
        for h in self.iter().rev() {
            ctx.push(h);
        }

        ctx.next().await
    }
}

/// Impl Handler for async function and closure
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// async fn free_function(_ctx: Context) -> FibraResult<Response> {
///     Ok("Hello World!".into())
/// }
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx_function = Context::new();
///     let ctx_closure = Context::new();
///
///     let fun_function = free_function;
///     let fun_closure = |_ctx: Context| async { Ok("It Works!".into()) };
///
///     let mut res_function = fun_function.handle(ctx_function).await?;
///     let mut res_closure = fun_closure.handle(ctx_closure).await?;
///
///     assert_eq!(res_function.body_all().await?, "Hello World!");
///     assert_eq!(res_closure.body_all().await?, "It Works!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl<F, R> Handler for F
    where
        F: Fn(Context) -> R + Send + Sync + 'static,
        R: Future<Output = FibraResult<Response>> + Send + 'static
{
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        self(ctx).await
    }
}

/// Impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::new();
///     let mut res = (Status::OK, mime::APPLICATION_JSON, "{}").handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
///     assert_eq!(res.body_all().await?, "{}");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for (Status, Mime, &'static str) {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((self.0, self.1.clone(), self.2).into())
    }
}

/// Impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::new();
///     let mut res = (Status::OK, "Hello World!").handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.body_all().await?, "Hello World!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for (Status, &'static str) {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((*self).into())
    }
}

/// Impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::new();
///     let mut res = (mime::APPLICATION_JSON, "{}").handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
///     assert_eq!(res.body_all().await?, "{}");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for (Mime, &'static str) {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((self.0.clone(), self.1).into())
    }
}

/// Impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::new();
///     let mut res = ().handle(ctx).await?;
///
///     assert_eq!(res.body_all().await?, "");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for () {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok(().into())
    }
}

/// Impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::new();
///     let res = Status::UNAUTHORIZED.handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::UNAUTHORIZED);
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for Status {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((*self).into())
    }
}

/// Impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::new();
///     let mut res = "Hello World!".handle(ctx).await?;
///
///     assert_eq!(res.body_all().await?, "Hello World!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for &'static str {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((*self).into())
    }
}
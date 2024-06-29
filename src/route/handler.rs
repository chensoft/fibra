//! HTTP Handler
use crate::route::*;
use crate::types::*;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(&self, ctx: Context) -> impl Into<Response>;
}

/// impl Handler for async function and closure
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
///     let ctx_function = Context::default();
///     let ctx_closure = Context::default();
///
///     let fun_function = free_function;
///     let fun_closure = |_ctx: Context| async { Ok("It Works!".into()) };
///
///     let mut res_function = fun_function.invoke(ctx_function).await?;
///     let mut res_closure = fun_closure.invoke(ctx_closure).await?;
///
///     assert_eq!(res_function.body_all().await.unwrap_or_default(), "Hello World!");
///     assert_eq!(res_closure.body_all().await.unwrap_or_default(), "It Works!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl<F, R, O> Handler for F
where
    F: Fn(Context) -> R + Send + Sync + 'static,
    R: Future<Output = O> + Send + 'static,
    O: Into<Response>
{
    #[inline]
    async fn handle(&self, ctx: Context) -> O {
        self(ctx).await
    }
}

/// impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = (Status::OK, mime::APPLICATION_JSON, "{}").invoke(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
///     assert_eq!(res.body_all().await.unwrap_or_default(), "{}");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for (Status, Mime, &'static str) {
    #[inline]
    async fn handle(&self, _ctx: Context) -> Response {
        (self.0, self.1.clone(), self.2).into()
    }
}

/// impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = (Status::OK, "Hello World!").invoke(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.body_all().await.unwrap_or_default(), "Hello World!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for (Status, &'static str) {
    #[inline]
    async fn handle(&self, _ctx: Context) -> Response {
        (*self).into()
    }
}

/// impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = (mime::APPLICATION_JSON, "{}").invoke(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
///     assert_eq!(res.body_all().await.unwrap_or_default(), "{}");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for (Mime, &'static str) {
    #[inline]
    async fn handle(&self, _ctx: Context) -> Response {
        (self.0.clone(), self.1).into()
    }
}

/// impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = ().invoke(ctx).await?;
///
///     assert_eq!(res.body_all().await.unwrap_or_default(), "");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for () {
    #[inline]
    async fn handle(&self, _ctx: Context) -> Response {
        ().into()
    }
}

/// impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let res = Status::UNAUTHORIZED.invoke(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::UNAUTHORIZED);
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for Status {
    #[inline]
    async fn handle(&self, _ctx: Context) -> Response {
        (*self).into()
    }
}

/// impl Handler for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = "Hello World!".invoke(ctx).await?;
///
///     assert_eq!(res.body_all().await.unwrap_or_default(), "Hello World!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Handler for &'static str {
    #[inline]
    async fn handle(&self, _ctx: Context) -> Response {
        (*self).into()
    }
}
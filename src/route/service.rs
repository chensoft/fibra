//! HTTP Service
use crate::route::*;
use crate::types::*;

/// The HTTP Service trait, service must implement this to process http requests
#[async_trait]
pub trait Service: AnyService + Send + Sync + 'static {
    /// Impl this function to handle http requests, a context contains a connection object and
    /// a current request object, multiple requests may reside on one connection, and these
    /// requests will be handled one by one on different context objects
    async fn handle(&self, ctx: Context) -> FibraResult<Response>;

    /// Internal method to get the child service of its parent
    #[inline]
    #[allow(unused_variables)]
    fn select(&self, idx: usize) -> Option<&dyn Service> {
        None
    }
}

/// Any support
pub trait AnyService: Any {
    /// Treat object as any
    fn as_any(&self) -> &dyn Any;

    /// Treat object as any mut
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AnyService for T {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Box Service
pub type BoxService = Box<dyn Service>;

#[async_trait]
impl Service for BoxService {
    #[inline]
    async fn handle<'a>(&'a self, ctx: Context) -> FibraResult<Response> {
        self.as_ref().handle(ctx).await
    }
}

/// As Service
///
/// # Examples
///
/// ```
/// use fibra::*;
/// use async_trait::*;
///
/// struct ServiceA;
/// struct ServiceB;
///
/// #[async_trait]
/// impl Service for ServiceA { async fn handle(&self, _ctx: Context) -> FibraResult<Response> { unimplemented!() } }
/// #[async_trait]
/// impl Service for ServiceB { async fn handle(&self, _ctx: Context) -> FibraResult<Response> { unimplemented!() } }
///
/// let mut service_a: BoxService = Box::new(ServiceA);
/// let mut service_b: BoxService = Box::new(ServiceB);
///
/// assert_eq!(service_a.as_service::<ServiceA>().is_some(), true);
/// assert_eq!(service_a.as_service::<ServiceB>().is_some(), false);
/// assert_eq!(service_b.as_service::<ServiceA>().is_some(), false);
/// assert_eq!(service_b.as_service::<ServiceB>().is_some(), true);
///
/// assert_eq!(service_a.as_service_mut::<ServiceA>().is_some(), true);
/// assert_eq!(service_a.as_service_mut::<ServiceB>().is_some(), false);
/// assert_eq!(service_b.as_service_mut::<ServiceA>().is_some(), false);
/// assert_eq!(service_b.as_service_mut::<ServiceB>().is_some(), true);
/// ```
pub trait AsService {
    /// Convert service if possible
    fn as_service<T: Service>(&self) -> Option<&T>;

    /// Convert service if possible
    fn as_service_mut<T: Service>(&mut self) -> Option<&mut T>;
}

impl AsService for BoxService {
    #[inline]
    fn as_service<T: Service>(&self) -> Option<&T> {
        self.as_ref().as_any().downcast_ref::<T>()
    }

    #[inline]
    fn as_service_mut<T: Service>(&mut self) -> Option<&mut T> {
        self.as_mut().as_any_mut().downcast_mut::<T>()
    }
}

/// Impl Service for vector
#[async_trait]
impl<T: Service> Service for Vec<T> {
    #[inline]
    async fn handle(&self, mut ctx: Context) -> FibraResult<Response> {
        ctx.push(self, true, 0);
        ctx.next().await
    }

    #[inline]
    fn select(&self, idx: usize) -> Option<&dyn Service> {
        self.get(idx).map(|service| service as &dyn Service)
    }
}

/// Impl Service for async function and closure
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
///     let mut res_function = fun_function.handle(ctx_function).await?;
///     let mut res_closure = fun_closure.handle(ctx_closure).await?;
///
///     assert_eq!(res_function.body_all().await.unwrap_or_default(), "Hello World!");
///     assert_eq!(res_closure.body_all().await.unwrap_or_default(), "It Works!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl<F, R> Service for F
    where
        F: Fn(Context) -> R + Send + Sync + 'static,
        R: Future<Output = FibraResult<Response>> + Send + 'static
{
    #[inline]
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        self(ctx).await
    }
}

/// Impl Service for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = (Status::OK, mime::APPLICATION_JSON, "{}").handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
///     assert_eq!(res.body_all().await.unwrap_or_default(), "{}");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Service for (Status, Mime, &'static str) {
    #[inline]
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((self.0, self.1.clone(), self.2).into())
    }
}

/// Impl Service for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = (Status::OK, "Hello World!").handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.body_all().await.unwrap_or_default(), "Hello World!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Service for (Status, &'static str) {
    #[inline]
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((*self).into())
    }
}

/// Impl Service for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = (mime::APPLICATION_JSON, "{}").handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::OK);
///     assert_eq!(res.header_ref(header::CONTENT_TYPE).map(|v| v.as_bytes()), Some(mime::APPLICATION_JSON.as_ref().as_bytes()));
///     assert_eq!(res.body_all().await.unwrap_or_default(), "{}");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Service for (Mime, &'static str) {
    #[inline]
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((self.0.clone(), self.1).into())
    }
}

/// Impl Service for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = ().handle(ctx).await?;
///
///     assert_eq!(res.body_all().await.unwrap_or_default(), "");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Service for () {
    #[inline]
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok(().into())
    }
}

/// Impl Service for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let res = Status::UNAUTHORIZED.handle(ctx).await?;
///
///     assert_eq!(res.status_ref(), &Status::UNAUTHORIZED);
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Service for Status {
    #[inline]
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((*self).into())
    }
}

/// Impl Service for static value
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let ctx = Context::default();
///     let mut res = "Hello World!".handle(ctx).await?;
///
///     assert_eq!(res.body_all().await.unwrap_or_default(), "Hello World!");
///
///     Ok(())
/// }
/// ```
#[async_trait]
impl Service for &'static str {
    #[inline]
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        Ok((*self).into())
    }
}
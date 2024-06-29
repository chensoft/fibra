//! HTTP Service
use crate::route::*;
use crate::types::*;

/// The HTTP Service trait, service must implement this to process http requests
#[async_trait]
pub trait Service: AnyService + Send + Sync + 'static {
    /// Impl this function to handle http requests, a context contains a connection object and
    /// a current request object, multiple requests may reside on one connection, and these
    /// requests will be handled one by one on different context objects
    async fn invoke(&self, ctx: Context) -> FibraResult<Response>;

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
    async fn invoke<'a>(&'a self, ctx: Context) -> FibraResult<Response> {
        self.as_ref().invoke(ctx).await
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
/// impl Service for ServiceA { async fn invoke(&self, _ctx: Context) -> FibraResult<Response> { unimplemented!() } }
/// #[async_trait]
/// impl Service for ServiceB { async fn invoke(&self, _ctx: Context) -> FibraResult<Response> { unimplemented!() } }
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
    async fn invoke(&self, mut ctx: Context) -> FibraResult<Response> {
        ctx.push(self, true, 0);
        ctx.next().await
    }

    #[inline]
    fn select(&self, idx: usize) -> Option<&dyn Service> {
        self.get(idx).map(|service| service as &dyn Service)
    }
}

/// Impl Service for server
#[async_trait]
impl<T: Handler> Service for T {
    #[inline]
    async fn invoke(&self, ctx: Context) -> FibraResult<Response> {
        Ok(self.handle(ctx).await.into())
    }
}
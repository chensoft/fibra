//! Catch Errors
use crate::types::*;

/// Catch failure responses and errors then call the service
pub struct Catcher {
    service: Arc<dyn Fn(Response, Option<FibraError>) -> Response + Send + Sync + 'static>,
}

impl Catcher {
    /// Create a new object
    #[inline]
    pub fn new() -> Self {
        let service = Arc::new(|res, _err| res);
        Self { service }
    }

    /// Set custom service
    #[inline]
    pub fn service<F>(&mut self, f: F) -> &mut Self where F: Fn(Response, Option<FibraError>) -> Response + Send + Sync + 'static {
        self.service = Arc::new(f);
        self
    }

    /// Catch the error or panic then turn it into a Response object
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> FibraResult<()> {
    ///     let catcher = Catcher::new();
    ///     assert_eq!(catcher.protect(async { Ok(Response::from("It Works!")) }).await.body_all().await.unwrap_or_default(), "It Works!");
    ///     assert_eq!(catcher.protect(async { panic!("Fatal Error") }).await.status_ref(), &Status::INTERNAL_SERVER_ERROR);
    ///     Ok(())
    /// }
    /// ```
    pub async fn protect<F>(&self, f: F) -> Response where F: Future<Output = FibraResult<Response>> {
        use futures::FutureExt;

        let service = self.service.clone();

        match AssertUnwindSafe(f).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(res) if res.status_ref().is_success() => res,
                Ok(res) => service(res, None),
                Err(err) => service(Status::INTERNAL_SERVER_ERROR.into(), Some(err)),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => service(Status::INTERNAL_SERVER_ERROR.into(), Some(FibraError::PanicError(err.to_string().into()))),
                None => service(Status::INTERNAL_SERVER_ERROR.into(), Some(FibraError::PanicError("Unknown panic".into()))),
            }
        }
    }
}

impl Default for Catcher {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Construct from custom error service
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let catcher = Catcher::from(|res: Response, err| {
///         match err {
///             Some(FibraError::PanicError(_)) => res.status(Status::SERVICE_UNAVAILABLE),
///             _ => res,
///         }
///     });
///
///     assert_eq!(catcher.protect(async { Ok(Response::from("It Works!")) }).await.status_ref(), &Status::OK);
///     assert_eq!(catcher.protect(async { Ok(Response::from("It Works!")) }).await.body_all().await.unwrap_or_default(), "It Works!");
///     assert_eq!(catcher.protect(async { Ok(Response::new().status(Status::NOT_FOUND)) }).await.status_ref(), &Status::NOT_FOUND);
///     assert_eq!(catcher.protect(async { panic!("Fatal Error") }).await.status_ref(), &Status::SERVICE_UNAVAILABLE);
///     assert_eq!(catcher.protect(async { Err(FibraError::RadixError(radixmap::RadixError::PathMalformed(""))) }).await.status_ref(), &Status::INTERNAL_SERVER_ERROR);
///
///     Ok(())
/// }
/// ```
impl<F> From<F> for Catcher
    where
        F: Fn(Response, Option<FibraError>) -> Response + Send + Sync + 'static
{
    #[inline]
    fn from(f: F) -> Self {
        Self { service: Arc::new(f) }
    }
}
//! Catch Errors
use crate::types::*;

/// Catch errors and call the handler
pub struct Catcher {
    /// Handler
    pub handler: Arc<dyn Fn(FibraError) -> Response + Send + Sync + 'static>,
}

impl Catcher {
    /// Set custom handler
    pub fn handler<F>(&mut self, f: F) -> &mut Self where F: Fn(FibraError) -> Response + Send + Sync + 'static {
        self.handler = Arc::new(f);
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
    ///     let catcher = Catcher::default();
    ///     assert_eq!(catcher.protect(async { Ok(Response::from("It Works!")) }).await.body_all().await?, "It Works!");
    ///     assert_eq!(catcher.protect(async { panic!("Fatal Error") }).await.status_ref(), &Status::INTERNAL_SERVER_ERROR);
    ///     Ok(())
    /// }
    /// ```
    pub async fn protect<F>(&self, f: F) -> Response where F: Future<Output = FibraResult<Response>> {
        use futures::FutureExt;

        let handler = self.handler.clone();

        match AssertUnwindSafe(f).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(res) => res,
                Err(err) => handler(err),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => handler(FibraError::PanicError(err.to_string().into())),
                None => handler(FibraError::PanicError("Unknown panic".into())),
            }
        }
    }
}

impl Default for Catcher {
    fn default() -> Self {
        let handler = Arc::new(|err| {
            match err {
                FibraError::PathNotFound => Status::NOT_FOUND.into(),
                _ => Status::INTERNAL_SERVER_ERROR.into()
            }
        });

        Self { handler }
    }
}

/// Construct from custom error handler
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// #[tokio::main]
/// async fn main() -> FibraResult<()> {
///     let catcher = Catcher::from(|err| {
///         match err {
///             FibraError::PathNotFound => Status::NOT_FOUND.into(),
///             FibraError::PanicError(_) => Status::SERVICE_UNAVAILABLE.into(),
///             _ => Status::INTERNAL_SERVER_ERROR.into()
///         }
///     });
///
///     assert_eq!(catcher.protect(async { Ok(Response::from("It Works!")) }).await.body_all().await?, "It Works!");
///     assert_eq!(catcher.protect(async { panic!("Fatal Error") }).await.status_ref(), &Status::SERVICE_UNAVAILABLE);
///     assert_eq!(catcher.protect(async { Err(FibraError::PathNotFound) }).await.status_ref(), &Status::NOT_FOUND);
///     assert_eq!(catcher.protect(async { Err(FibraError::RadixError(radixmap::RadixError::PathMalformed(""))) }).await.status_ref(), &Status::INTERNAL_SERVER_ERROR);
///     assert_eq!(catcher.protect(async { panic!("abc") }).await.status_ref(), &Status::SERVICE_UNAVAILABLE);
///
///     Ok(())
/// }
/// ```
impl<F> From<F> for Catcher
    where
        F: Fn(FibraError) -> Response + Send + Sync + 'static
{
    fn from(f: F) -> Self {
        Self { handler: Arc::new(f) }
    }
}
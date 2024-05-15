//! Catch Errors
use crate::types::*;

/// Catch errors and call the handler
pub struct Catcher {
    /// Preset handler. **Do not assume the Response as it may change without notice**
    #[allow(clippy::type_complexity)]
    pub preset: Box<dyn Fn(FibraError) -> Response + Send + Sync + 'static>,

    /// Custom handler
    #[allow(clippy::type_complexity)]
    pub custom: Box<dyn Fn(&Catcher, FibraError) -> Response + Send + Sync + 'static>,
}

impl Catcher {
    /// Set preset handler
    pub fn preset<F>(&mut self, f: F) -> &mut Self where F: Fn(FibraError) -> Response + Send + Sync + 'static {
        self.preset = Box::new(f);
        self
    }

    /// Set custom handler
    pub fn custom<F>(&mut self, f: F) -> &mut Self where F: Fn(&Catcher, FibraError) -> Response + Send + Sync + 'static {
        self.custom = Box::new(f);
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

        match AssertUnwindSafe(f).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(res) => res,
                Err(err) => (self.custom)(self, err),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => (self.custom)(self, FibraError::PanicError(err.to_string().into())),
                None => (self.custom)(self, FibraError::PanicError("Unknown panic".into())),
            }
        }
    }
}

impl Default for Catcher {
    fn default() -> Self {
        let preset = Box::new(|err| {
            match err {
                FibraError::PathNotFound => Status::NOT_FOUND.into(),
                _ => Status::INTERNAL_SERVER_ERROR.into()
            }
        });

        Self { preset, custom: Box::new(|obj, err| (obj.preset)(err)) }
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
///     let catcher = Catcher::from(|obj, err| {
///         match err {
///             FibraError::PanicError(_) => Status::SERVICE_UNAVAILABLE.into(),
///             _ => obj.default(err)
///         }
///     });
///
///     assert_eq!(catcher.protect(async { Ok(Response::from("It Works!")) }).await.body_all().await?, "It Works!");
///     assert_eq!(catcher.protect(async { panic!("Fatal Error") }).await.status_ref(), &Status::SERVICE_UNAVAILABLE);
///     assert_eq!(catcher.protect(async { Err(FibraError::PathDuplicate("/".into())) }).await.status_ref(), &Status::INTERNAL_SERVER_ERROR);
///
///     Ok(())
/// }
/// ```
impl<F> From<F> for Catcher
    where
        F: Fn(&Catcher, FibraError) -> Response + Send + Sync + 'static
{
    fn from(f: F) -> Self {
        Self { custom: Box::new(f), ..Default::default() }
    }
}
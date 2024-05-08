//! Catch Errors
use crate::route::*;
use crate::types::*;

/// Catch errors and call the handler
pub struct Catcher {
    /// Preset handler. **Do not assume the Response as it may change without notice**
    pub preset: Box<dyn Fn(BoltError) -> Response + Send + Sync + 'static>,

    /// Custom handler
    pub custom: Box<dyn Fn(&Catcher, BoltError) -> Response + Send + Sync + 'static>,
}

impl Catcher {
    /// Set preset handler
    pub fn preset(mut self, f: impl Fn(BoltError) -> Response + Send + Sync + 'static) -> Self {
        self.preset = Box::new(f);
        self
    }

    /// Set custom handler
    pub fn custom(mut self, f: impl Fn(&Catcher, BoltError) -> Response + Send + Sync + 'static) -> Self {
        self.custom = Box::new(f);
        self
    }

    /// Catch the panic and turn into an error object
    ///
    /// # Examples
    /// 
    /// ```
    /// use bolt::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> BoltResult<()> {
    ///     let catcher = addon::Catcher::default();
    ///     assert_eq!(catcher.catch(async { Ok(Response::from("It Works!")) }).await, Ok(Response::from("It Works!")));
    ///     assert_eq!(catcher.catch(async { panic!("Fatal Error") }).await, Err(BoltError::PanicError("Fatal Error".into())));
    ///     Ok(())
    /// }
    /// ```
    pub async fn catch<F: Future<Output = BoltResult<Response>>>(&self, f: F) -> BoltResult<Response> {
        use futures::FutureExt;

        match AssertUnwindSafe(f).catch_unwind().await {
            Ok(ret) => match ret {
                Ok(res) => Ok(res),
                Err(err) => Err(err),
            }
            Err(err) => match err.downcast_ref::<&str>() {
                Some(err) => Err(BoltError::PanicError(err.to_string().into())),
                None => Err(BoltError::PanicError("Unknown panic".into())),
            }
        }
    }
}

impl Default for Catcher {
    fn default() -> Self {
        let preset = Box::new(|_| {
            Status::INTERNAL_SERVER_ERROR.into()
        });

        Self { preset, custom: Box::new(|obj, err| (obj.preset)(err)) }
    }
}

/// Construct from custom error handler
///
/// ```
/// use bolt::*;
///
/// let _ = addon::Catcher::from(|obj, err| {
///     match err {
///         BoltError::PanicError(_) => Status::SERVICE_UNAVAILABLE.into(),
///         _ => obj.default(err)
///     }
/// });
/// ```
impl<F> From<F> for Catcher
    where
        F: Fn(&Catcher, BoltError) -> Response + Send + Sync + 'static
{
    fn from(f: F) -> Self {
        Self { custom: Box::new(f), ..Default::default() }
    }
}

#[async_trait]
impl Handler for Catcher {
    async fn handle(&self, ctx: Context) -> BoltResult<Response> {
        match self.catch(ctx.next()).await {
            Ok(res) => Ok(res),
            Err(err) => Ok((self.custom)(self, err)),
        }
    }
}
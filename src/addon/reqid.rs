//! ReqID Middleware
use crate::route::*;
use crate::types::*;

/// ReqID Middleware
pub struct ReqID {
    header: String,
}

impl ReqID {
    /// Create a new object
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// app.mount(addon::ReqID::new());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { header: "x-request-id".to_string() }
    }

    /// Set request id's header name
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// app.mount(addon::ReqID::new().header("x-request-id"));
    /// ```
    #[inline]
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = header.into();
        self
    }
}

impl Default for ReqID {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Service for ReqID {
    async fn invoke(&self, mut ctx: Context) -> FibraResult<Response> {
        let key = HeaderName::try_from(self.header.as_str())?;
        let val = ulid::Ulid::from_datetime(*ctx.created()).to_string();

        ctx.req_mut().headers_mut().insert(key.clone(), val.clone());
        let mut res = ctx.next().await;

        if let Ok(res) = &mut res {
            res.headers_mut().insert(key, val);
        }

        res
    }
}
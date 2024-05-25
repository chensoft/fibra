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
    pub fn new() -> Self {
        Self { header: "X-Request-ID".to_string() }
    }

    /// Set request id's header name
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// app.mount(addon::ReqID::new().name("X-Request-ID"));
    /// ```
    pub fn header(mut self, name: impl Into<String>) -> Self {
        self.header = name.into();
        self
    }
}

impl Default for ReqID {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Handler for ReqID {
    async fn handle(&self, mut ctx: Context) -> FibraResult<Response> {
        let key = HeaderName::try_from(self.header.as_str())?;
        let val = HeaderValue::try_from(ulid::Ulid::from_datetime(*ctx.created()).to_string())?;

        ctx.req_mut().headers_mut().insert(key.clone(), val.clone());
        let mut res = ctx.next().await;

        if let Ok(res) = &mut res {
            res.headers_mut().insert(key, val);
        }

        res
    }
}
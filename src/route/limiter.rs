//! HTTP Filter
use crate::route::*;
use crate::types::*;

/// Limiter is a struct used for filtering HTTP requests
#[derive(Default)]
pub struct Limiter {
    /// Store all filters
    pub limits: Vec<Box<dyn Fn(&Context) -> Status + Send + Sync + 'static>>
}

impl Limiter {
    /// Insert a filter
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().insert(|_| Status::FORBIDDEN);
    /// let context = Context::from(Request::default());
    ///
    /// assert_eq!(limiter.filter(&context), Status::FORBIDDEN);
    /// ```
    pub fn insert<F>(&mut self, f: F) -> &mut Self where F: Fn(&Context) -> Status + Send + Sync + 'static {
        self.limits.push(Box::new(f));
        self
    }

    /// Execute filters and return the first Non-OK result
    pub fn filter(&self, ctx: &Context) -> Status {
        self.limits.iter().find_map(|f| {
            let status = f(ctx);
            match status == Status::OK {
                true => None,
                false => Some(status),
            }
        }).unwrap_or(Status::OK)
    }

    /// Clear the limiter
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().insert(|_| Status::FORBIDDEN);
    /// let context = Context::from(Request::default());
    ///
    /// assert_eq!(limiter.filter(&context), Status::FORBIDDEN);
    ///
    /// limiter.clear();
    ///
    /// assert_eq!(limiter.filter(&context), Status::OK);
    /// ```
    pub fn clear(&mut self) -> &mut Self {
        self.limits.clear();
        self
    }
}

impl Limiter {
    /// Limit method
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().method(Method::PUT);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::PUT))), Status::OK);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::GET))), Status::METHOD_NOT_ALLOWED);
    /// ```
    pub fn method(&mut self, val: impl Into<Method>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| match ctx.method() == val {
            true => Status::OK,
            false => Status::METHOD_NOT_ALLOWED
        })
    }

    /// Limit multiple methods, any pass will do
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().methods(vec![Method::PUT, Method::PATCH]);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::PUT))), Status::OK);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::PATCH))), Status::OK);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::GET))), Status::METHOD_NOT_ALLOWED);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::POST))), Status::METHOD_NOT_ALLOWED);
    /// ```
    pub fn methods(&mut self, vec: Vec<impl Into<Method>>) -> &mut Self {
        let vec: Vec<Method> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| match vec.iter().any(|val| ctx.method() == val ) {
            true => Status::OK,
            false => Status::METHOD_NOT_ALLOWED
        })
    }

    pub fn domain(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| match ctx.domain() == val {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn domains(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| match vec.iter().any(|val| ctx.domain() == val ) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn subdomain(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| match ctx.subdomain() == val {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn subdomains(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| match vec.iter().any(|val| ctx.subdomain() == val ) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn host(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| match ctx.host() == val {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn hosts(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| match vec.iter().any(|val| ctx.host() == val ) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn path(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| match ctx.path() == val {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn paths(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| match vec.iter().any(|val| ctx.path() == val ) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn query(&mut self, key: impl Into<String>, val: impl Into<String>) -> &mut Self {
        let key = key.into();
        let val = val.into();

        self.insert(move |ctx| match ctx.query(key.as_str()) == val {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn queries(&mut self, vec: Vec<(impl Into<String>, impl Into<String>)>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(|(key, val)| (key.into(), val.into())).collect();

        self.insert(move |ctx| match vec.iter().any(|(key, val)| ctx.query(key.as_str()) == val ) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn header(&mut self, key: impl Into<HeaderName>, val: impl Into<HeaderValue>) -> &mut Self {
        let key = key.into();
        let val = val.into();

        self.insert(move |ctx| match ctx.header(&key) == Some(&val) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn headers(&mut self, vec: Vec<(impl Into<HeaderName>, impl Into<HeaderValue>)>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(|(key, val)| (key.into(), val.into())).collect();

        self.insert(move |ctx| match vec.iter().any(|(key, val)| ctx.header(key) == Some(val) ) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn param(&mut self, key: impl Into<String>, val: impl Into<String>) -> &mut Self {
        let key = key.into();
        let val = val.into();

        self.insert(move |ctx| match ctx.param(key.as_str()) == val {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }

    pub fn params(&mut self, vec: Vec<(impl Into<String>, impl Into<String>)>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(|(key, val)| (key.into(), val.into())).collect();

        self.insert(move |ctx| match vec.iter().any(|(key, val)| ctx.param(key.as_str()) == val ) {
            true => Status::OK,
            false => Status::NOT_FOUND
        })
    }
}
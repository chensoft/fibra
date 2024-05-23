//! HTTP Filter
use crate::route::*;
use crate::types::*;

/// Limiter is a struct used for filtering HTTP requests
#[derive(Default)]
pub struct Limiter {
    /// Store all filters
    #[allow(clippy::type_complexity)]
    limits: Vec<Box<dyn Fn(&Context) -> bool + Send + Sync + 'static>>
}

impl Limiter {
    /// Create a new object
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a filter
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// let context = Context::new();
    ///
    /// limiter.push(|ctx| ctx.is_get());
    ///
    /// assert_eq!(limiter.test(&context), true);
    /// ```
    pub fn push<F>(&mut self, f: F) -> &mut Self where F: Fn(&Context) -> bool + Send + Sync + 'static {
        self.limits.push(Box::new(f));
        self
    }

    /// Check filters and return the first false result
    pub fn test(&self, ctx: &Context) -> bool {
        self.limits.iter().find_map(|f| {
            let pass = f(ctx);
            match pass {
                true => None,
                false => Some(false),
            }
        }).unwrap_or(true)
    }

    /// Clear the limiter
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// let context = Context::new();
    ///
    /// limiter.push(|_| false);
    ///
    /// assert_eq!(limiter.test(&context), false);
    ///
    /// limiter.clear();
    ///
    /// assert_eq!(limiter.test(&context), true);
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
    /// let mut limiter = Limiter::new();
    /// limiter.method(Method::PUT);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().method(Method::PUT))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().method(Method::GET))), false);
    /// ```
    pub fn method(&mut self, val: impl Into<Method>) -> &mut Self {
        let val = val.into();

        self.push(move |ctx| ctx.method() == val)
    }

    /// Limit multiple methods, any pass will do
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.methods(vec![Method::PUT, Method::PATCH]);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().method(Method::PUT))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().method(Method::PATCH))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().method(Method::GET))), false);
    /// assert_eq!(limiter.test(&Context::from(Request::new().method(Method::POST))), false);
    /// ```
    pub fn methods(&mut self, vec: Vec<impl Into<Method>>) -> &mut Self {
        let vec: Vec<Method> = vec.into_iter().map(Into::into).collect();

        self.push(move |ctx| vec.iter().any(|val| ctx.method() == val))
    }

    /// Limit domain name
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.domain("localip.cc");
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.example.net"))), false);
    /// ```
    pub fn domain(&mut self, val: impl Into<Bytes>) -> &mut Self {
        let val = val.into();

        self.push(move |ctx| ctx.domain() == val)
    }

    /// Limit domain names
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.domains(vec!["localip.cc", "example.net"]);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.example.net"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.example.org"))), false);
    /// ```
    pub fn domains(&mut self, vec: Vec<impl Into<Bytes>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.push(move |ctx| vec.iter().any(|val| ctx.domain() == val))
    }

    /// Limit subdomain
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.subdomain("api");
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.localip.cc"))), false);
    /// ```
    pub fn subdomain(&mut self, val: impl Into<Bytes>) -> &mut Self {
        let val = val.into();

        self.push(move |ctx| ctx.subdomain() == val)
    }

    /// Limit subdomains
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.subdomains(vec!["api", "app"]);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://app.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.localip.cc"))), false);
    /// ```
    pub fn subdomains(&mut self, vec: Vec<impl Into<Bytes>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.push(move |ctx| vec.iter().any(|val| ctx.subdomain() == val))
    }

    /// Limit host
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.host("api.localip.cc");
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://app.localip.cc"))), false);
    /// ```
    pub fn host(&mut self, val: impl Into<Bytes>) -> &mut Self {
        let val = val.into();

        self.push(move |ctx| ctx.host() == val)
    }

    /// Limit hosts
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.hosts(vec!["api.localip.cc", "app.localip.cc"]);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://app.localip.cc"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://www.localip.cc"))), false);
    /// ```
    pub fn hosts(&mut self, vec: Vec<impl Into<Bytes>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.push(move |ctx| vec.iter().any(|val| ctx.host() == val))
    }

    /// Limit path
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.path("/user/12345");
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/user/12345"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/user/abcde"))), false);
    /// ```
    pub fn path(&mut self, val: impl Into<Bytes>) -> &mut Self {
        let val = val.into();

        self.push(move |ctx| ctx.path() == val)
    }

    /// Limit paths
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.paths(vec!["/user/12345", "/user/abcde"]);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/user/12345"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/user/abcde"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/user/qwert"))), false);
    /// ```
    pub fn paths(&mut self, vec: Vec<impl Into<Bytes>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.push(move |ctx| vec.iter().any(|val| ctx.path() == val))
    }

    /// Limit query
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.query("id", "12345");
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/?id=12345"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/?id=abcde"))), false);
    /// ```
    pub fn query(&mut self, key: impl Into<String>, val: impl Into<Bytes>) -> &mut Self {
        let key = key.into();
        let val = val.into();

        self.push(move |ctx| ctx.query(key.as_str()) == val)
    }

    /// Limit queries
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.queries(vec![("id", "12345"), ("id", "abcde")]);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/?id=12345"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/?id=abcde"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().uri("http://api.localip.cc/?id=qwert"))), false);
    /// ```
    pub fn queries(&mut self, vec: Vec<(impl Into<String>, impl Into<Bytes>)>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(|(key, val)| (key.into(), val.into())).collect();

        self.push(move |ctx| vec.iter().any(|(key, val)| ctx.query(key.as_str()) == val))
    }

    /// Limit header
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.header("content-type", "text/plain");
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().header("content-type", "text/plain"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().header("content-type", "application/json"))), false);
    /// ```
    pub fn header(&mut self, key: impl IntoHeaderName, val: impl IntoHeaderValue) -> &mut Self {
        let key = key.into_header_name();
        let val = val.into_header_value();
        self.push(move |ctx| ctx.header(&key) == Some(&val))
    }

    /// Limit headers
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut limiter = Limiter::new();
    /// limiter.headers(vec![("content-type", "text/plain"), ("content-type", "application/json")]);
    ///
    /// assert_eq!(limiter.test(&Context::from(Request::new().header("content-type", "text/plain"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().header("content-type", "application/json"))), true);
    /// assert_eq!(limiter.test(&Context::from(Request::new().header("content-type", "application/yaml"))), false);
    /// ```
    pub fn headers(&mut self, vec: Vec<(impl IntoHeaderName, impl IntoHeaderValue)>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(|(key, val)| (key.into_header_name(), val.into_header_value())).collect();

        self.push(move |ctx| vec.iter().any(|(key, val)| ctx.header(key) == Some(val)))
    }
}
//! HTTP Filter
use crate::route::*;
use crate::types::*;

/// Limiter is a struct used for filtering HTTP requests
#[derive(Default)]
pub struct Limiter {
    /// Store all filters
    #[allow(clippy::type_complexity)]
    pub limits: Vec<Box<dyn Fn(&Context) -> bool + Send + Sync + 'static>>
}

impl Limiter {
    /// Insert a filter
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().insert(|ctx| ctx.is_get());
    /// let context = Context::from(Request::default());
    ///
    /// assert_eq!(limiter.filter(&context), true);
    /// ```
    pub fn insert<F>(&mut self, f: F) -> &mut Self where F: Fn(&Context) -> bool + Send + Sync + 'static {
        self.limits.push(Box::new(f));
        self
    }

    /// Check filters and return the first false result
    pub fn filter(&self, ctx: &Context) -> bool {
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
    /// let limiter = Limiter::default().insert(|_| false);
    /// let context = Context::from(Request::default());
    ///
    /// assert_eq!(limiter.filter(&context), false);
    ///
    /// limiter.clear();
    ///
    /// assert_eq!(limiter.filter(&context), true);
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
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::PUT))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::GET))), false);
    /// ```
    pub fn method(&mut self, val: impl Into<Method>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| ctx.method() == val)
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
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::PUT))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::PATCH))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::GET))), false);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().method(Method::POST))), false);
    /// ```
    pub fn methods(&mut self, vec: Vec<impl Into<Method>>) -> &mut Self {
        let vec: Vec<Method> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| vec.iter().any(|val| ctx.method() == val))
    }

    /// Limit domain name
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().domain("example.com");
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.net"))), false);
    /// ```
    pub fn domain(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| ctx.domain() == val)
    }

    /// Limit domain names
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().domains(vec!["example.com", "example.net"]);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.net"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.org"))), false);
    /// ```
    pub fn domains(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| vec.iter().any(|val| ctx.domain() == val))
    }

    /// Limit subdomain
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().subdomain("api");
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.com"))), false);
    /// ```
    pub fn subdomain(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| ctx.subdomain() == val)
    }

    /// Limit subdomains
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().subdomains(vec!["api", "app"]);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://app.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.com"))), false);
    /// ```
    pub fn subdomains(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| vec.iter().any(|val| ctx.subdomain() == val))
    }

    /// Limit host
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().host("api.example.com");
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://app.example.com"))), false);
    /// ```
    pub fn host(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| ctx.host() == val)
    }

    /// Limit hosts
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().hosts(vec!["api.example.com", "app.example.com"]);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://app.example.com"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://www.example.com"))), false);
    /// ```
    pub fn hosts(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| vec.iter().any(|val| ctx.host() == val))
    }

    /// Limit path
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().path("/user/12345");
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/user/12345"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/user/abcde"))), false);
    /// ```
    pub fn path(&mut self, val: impl Into<String>) -> &mut Self {
        let val = val.into();

        self.insert(move |ctx| ctx.path() == val)
    }

    /// Limit paths
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().paths(vec!["/user/12345", "/user/abcde"]);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/user/12345"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/user/abcde"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/user/qwert"))), false);
    /// ```
    pub fn paths(&mut self, vec: Vec<impl Into<String>>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(Into::into).collect();

        self.insert(move |ctx| vec.iter().any(|val| ctx.path() == val))
    }

    /// Limit query
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().query("id", "12345");
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/?id=12345"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/?id=abcde"))), false);
    /// ```
    pub fn query(&mut self, key: impl Into<String>, val: impl Into<String>) -> &mut Self {
        let key = key.into();
        let val = val.into();

        self.insert(move |ctx| ctx.query(key.as_str()) == val)
    }

    /// Limit queries
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().queries(vec![("id", "12345"), ("id", "abcde")]);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/?id=12345"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/?id=abcde"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().uri("http://api.example.com/?id=qwert"))), false);
    /// ```
    pub fn queries(&mut self, vec: Vec<(impl Into<String>, impl Into<String>)>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(|(key, val)| (key.into(), val.into())).collect();

        self.insert(move |ctx| vec.iter().any(|(key, val)| ctx.query(key.as_str()) == val))
    }

    /// Limit header
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().header("content-type", "text/plain");
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().header("content-type", "text/plain"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().header("content-type", "application/json"))), false);
    /// ```
    pub fn header(&mut self, key: impl Into<HeaderName>, val: impl Into<HeaderValue>) -> &mut Self {
        let key = key.into();
        let val = val.into();

        self.insert(move |ctx| ctx.header(&key) == Some(&val))
    }

    /// Limit headers
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let limiter = Limiter::default().headers(vec![("content-type", "text/plain"), ("content-type", "application/json")]);
    ///
    /// assert_eq!(limiter.filter(&Context::from(Request::default().header("content-type", "text/plain"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().header("content-type", "application/json"))), true);
    /// assert_eq!(limiter.filter(&Context::from(Request::default().header("content-type", "application/yaml"))), false);
    /// ```
    pub fn headers(&mut self, vec: Vec<(impl Into<HeaderName>, impl Into<HeaderValue>)>) -> &mut Self {
        let vec: Vec<_> = vec.into_iter().map(|(key, val)| (key.into(), val.into())).collect();

        self.insert(move |ctx| vec.iter().any(|(key, val)| ctx.header(key) == Some(val)))
    }
}
use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Route {
    pub pattern: Pattern,
    pub handler: Box<dyn Handler>,
}

impl Route {
    pub fn new(pattern: impl Into<Pattern>, handler: impl Handler) -> Self {
        Self {pattern: pattern.into(), handler: Box::new(handler)}
    }
}

// todo derive
pub struct Filter {
    pub routes: Vec<Route>
}

impl Filter {
    pub fn new(pattern: impl Into<Pattern>, handler: impl Handler) -> Self {
        Self {routes: vec![Route::new(pattern, handler)]}
    }

    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Self {
        self.routes.push(Route::new(pattern, handler));
        self
    }
}

/// Function Wrapper
pub struct Func<F, R>
    where
        F: Fn(Context) -> R + Sync + Send + 'static,
        R: Future<Output = Result<()>> + Sync + Send + 'static
{
    pub f: F
}

#[async_trait]
impl<F, R> Handler for Func<F, R>
    where
        F: Fn(Context) -> R + Sync + Send + 'static,
        R: Future<Output = Result<()>> + Sync + Send + 'static
{
    async fn handle(&self, ctx: Context) -> Result<()> {
        (self.f)(ctx).await
    }
}

impl<F, R> Func<F, R>
    where
        F: Fn(Context) -> R + Sync + Send + 'static,
        R: Future<Output = Result<()>> + Sync + Send + 'static
{
    pub fn new(f: F) -> Self {
        Func {f}
    }
}

/// Any
pub struct Any;

#[async_trait]
impl Handler for Any {
    async fn handle(&self, mut ctx: Context) -> Result<()> {
        ctx.next()
    }
}

/// Get
pub struct Get;

#[async_trait]
impl Handler for Get {
    async fn handle(&self, mut ctx: Context) -> Result<()> {
        // todo re-route?
        match ctx.req.method() == http::Method::GET {
            true => ctx.next(),
            false => ctx.next(),
        }
    }
}
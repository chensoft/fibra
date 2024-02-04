use crate::*;

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
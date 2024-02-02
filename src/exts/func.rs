use crate::core::*;

struct Func<F, R>
    where
        F: Fn(Context) -> R + Sync + Send + 'static,
        R: Future<Output = Result<()>> + Sync + Send + 'static
{
    f: F
}

#[async_trait]
impl<F, R> Handler for Func<F, R>
    where
        F: Fn(Context) -> R + Sync + Send + 'static,
        R: Future<Output = Result<()>> + Sync + Send + 'static
{
    async fn handle(&mut self, ctx: Context) -> Result<()> {
        (self.f)(ctx).await
    }
}

pub fn wrap<F, R>(f: F) -> impl Handler
    where
        F: Fn(Context) -> R + Sync + Send + 'static,
        R: Future<Output = Result<()>> + Sync + Send + 'static
{
    Func {f}
}
use crate::consts::*;
use crate::kernel::*;

pub struct Closure<F>
    where
        F: for<'a> Fn(&'a mut Context) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'static,
{
    pub closure: F,
}
// todo impl From to Closure?
impl<F> Closure<F>
    where
        F: for<'a> Fn(&'a mut Context) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'static,
{
    pub fn new(closure: F) -> Self {
        Self {closure}
    }
}

#[async_trait]
impl<F> Handler for Closure<F>
    where
        F: for<'a> Fn(&'a mut Context) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'static,
{
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        (self.closure)(ctx).await
    }
}
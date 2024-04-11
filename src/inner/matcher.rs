use crate::inner::*;

#[derive(Default)]
pub struct Matcher {
    pub preway: HashMap<Pattern, Package>
}

impl Matcher {
    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Routine {
        let pattern = pattern.into();
        let package = self.preway.entry(pattern.clone()).or_default();
        package.insert(Routine::new(handler))
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, ctx: Context) -> FibraResult<Response<Body>> {
        match self.preway.get(&Pattern) {
            Some(pkg) => pkg.handle(ctx).await,
            None => ctx.next().await
        }
    }
}
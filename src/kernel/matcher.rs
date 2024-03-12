use crate::kernel::*;

#[derive(Default)]
pub struct Matcher {
    pub preway: HashMap<Pattern, Package>
}

impl Matcher {
    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Routine {
        let pattern = pattern.into();
        let package = self.preway.entry(pattern.clone()).or_insert(Package::default());
        package.insert(Routine::new(handler))
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn warmup(&mut self) -> Result<()> {
        for p in self.preway.values_mut() {
            p.warmup().await?;
        }

        Ok(())
    }

    async fn handle(&self, ctx: Context) -> Result<Response<Body>> {
        match self.preway.get(&Pattern::default()) {
            Some(pkg) => pkg.handle(ctx).await,
            None => ctx.next().await
        }
    }
}
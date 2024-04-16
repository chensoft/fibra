use crate::types::*;
use crate::route::*;

#[derive(Default)]
pub struct Matcher {
    pub routes: RadixMap<'static, Package>
}

impl Matcher {
    pub fn add(&mut self, pattern: &'static str, handler: impl Handler) -> FibraResult<&mut Routine> {
        if !self.routes.contains_key(pattern) {
            self.routes.insert(pattern, Package::default())?;
        }

        match self.routes.get_mut(pattern) {
            Some(package) => Ok(package.insert(Routine::new(handler))),
            None => unreachable!()
        }
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response<Body>> {
        // match self.preway.get(&Pattern) {
        //     Some(pkg) => pkg.handle(ctx).await,
        //     None => ctx.next().await
        // }
        todo!()
    }
}
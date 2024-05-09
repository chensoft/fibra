use crate::route::*;
use crate::types::*;

#[derive(Default)]
pub struct Matcher {
    pub routes: RadixMap<Routine>
}

impl Matcher {
    pub fn insert(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Routine> {
        // if !self.routes.contains_key(path) {
        //     self.routes.insert(path, Routine::default())?;
        // }
        //
        // match self.routes.get_mut(path) {
        //     Some(package) => Ok(package.insert(Routine::new(handler))),
        //     None => unreachable!()
        // }
        todo!()
    }

    pub fn matches() -> Option<()> {
        todo!()
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, _ctx: Context) -> FibraResult<Response> {
        // match self.preway.get(&path) {
        //     Some(pkg) => pkg.handle(ctx).await,
        //     None => ctx.next().await
        // }
        todo!()
    }
}
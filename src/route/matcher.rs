use crate::route::*;
use crate::types::*;

#[derive(Default)]
pub struct Matcher {
    routes: RadixMap<Routine>
}

impl Matcher {
    pub fn insert(&mut self, path: &'static str, handler: impl Handler) -> FibraResult<&mut Routine> {
        if self.routes.contains_key(path.as_bytes()) {
            return Err(FibraError::PathDuplicate(path.into()));
        }

        self.routes.insert(path, Routine::from(handler))?;
        Ok(self.routes.get_mut(path.as_bytes()).unwrap_or_else(|| unreachable!()))
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
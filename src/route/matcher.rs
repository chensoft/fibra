use crate::route::*;
use crate::types::*;

#[derive(Default)]
pub struct Matcher {
    routes: RadixMap<Vec<Routine>>
}

impl Matcher {
    pub fn insert(&mut self, path: impl Into<String>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let path = path.into();

        if !self.routes.contains_key(path.as_ref()) {
            self.routes.insert(path.clone(), vec![])?; // todo use entry or_insert
        }

        let list = self.routes.get_mut(path.as_ref()).unwrap_or_else(|| unreachable!());
        list.push(Routine::from(handler));

        Ok(list.last_mut().unwrap_or_else(|| unreachable!()))
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
use crate::inner::*;

#[derive(Default)]
pub struct Matcher {
    pub map: RadixMap<'static, Package>
}

impl Matcher {
    pub fn add(&mut self, pattern: &'static str, handler: impl Handler) -> FibraResult<&mut Routine> {
        if !self.map.contains_key(pattern) {
            self.map.insert(pattern, Package::default())?;
        }

        match self.map.get_mut(pattern) {
            Some(package) => Ok(package.insert(Routine::new(handler))),
            None => unreachable!()
        }
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, ctx: Context) -> FibraResult<Response<Body>> {
        // match self.preway.get(&Pattern) {
        //     Some(pkg) => pkg.handle(ctx).await,
        //     None => ctx.next().await
        // }
        todo!()
    }
}
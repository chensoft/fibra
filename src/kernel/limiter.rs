use crate::kernel::*;

#[derive(Default)]
pub struct Limiter {
//     pub limits: Vec<Box<dyn Fn(&Context) -> bool + Send + Sync + 'static>>
}

// impl Limiter {
//     pub fn add(&mut self, limit: impl Fn(&Context) -> bool + Send + Sync + 'static) -> &mut Self {
//         self.limits.push(Box::new(limit));
//         self
//     }
// }

#[async_trait]
impl Handler for Limiter {
    async fn handle(&self, _ctx: Context) -> Result<Response<Body>> {
//         if !self.limits.iter().all(|f| f(ctx)) {
//             ctx.skip();
//         }
// 
//         ctx.next().await
        todo!()
    }
}
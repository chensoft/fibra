use crate::route::*;
use crate::types::*;

pub struct Router {
    routes: [RadixMap<BoxHandler>; 8], // Method -> Handlers
}

impl Router {
    pub fn add(&mut self, path: &'static str, handler: impl Handler, method: Option<Method>) -> FibraResult<&mut Self> {
        Ok(self)
    }
}

impl Default for Router {
    fn default() -> Self {
        todo!()
    }
}

#[async_trait]
impl Handler for Router {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        todo!()
    }
}
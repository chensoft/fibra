use crate::*;

/// Any
pub struct Any;

#[async_trait]
impl Handler for Any {
    async fn handle(&mut self, mut ctx: Context) -> Result<()> {
        ctx.next()
    }
}

/// Get
pub struct Get;

#[async_trait]
impl Handler for Get {
    async fn handle(&mut self, mut ctx: Context) -> Result<()> {
        // todo re-route?
        match ctx.req.method() == http::Method::GET {
            true => ctx.next(),
            false => ctx.next(),
        }
    }
}
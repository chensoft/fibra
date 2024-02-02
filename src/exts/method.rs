use crate::core::*;

/// Any
struct Any;

#[async_trait]
impl Handler for Any {
    async fn handle(&mut self, mut ctx: Context) -> Result<()> {
        ctx.next()
    }
}

pub fn any() -> impl Handler {
    Any
}

/// Get
struct Get;

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

pub fn get() -> impl Handler {
    Get
}
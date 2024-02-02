use crate::core::Result;
use crate::core::Context;
use crate::core::Handler;

/// Any
struct Any;

impl Handler for Any {
    fn handle(&mut self, mut ctx: Context) -> Result<()> {
        ctx.next()
    }
}

pub fn any() -> impl Handler {
    Any
}

/// Get
struct Get;

impl Handler for Get {
    fn handle(&mut self, mut ctx: Context) -> Result<()> {
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
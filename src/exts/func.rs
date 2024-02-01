use std::future::Future;
use crate::core::Result;
use crate::core::Context;
use crate::core::Handler;

pub struct Func {
    pub func: Box<dyn FnMut(Context) -> dyn Future<Output = Result<()>>>,
}

impl Handler for Func {
    fn handle(&mut self, mut ctx: Context) -> Result<()> {
        ctx.next()
    }
}
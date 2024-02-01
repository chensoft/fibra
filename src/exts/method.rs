use crate::core::Result;
use crate::core::Context;
use crate::core::Handler;

pub struct Any;

impl Handler for Any {
    fn handle(&mut self, mut ctx: Context) -> Result<()> {
        ctx.next()
    }
}
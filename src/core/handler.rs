use super::types::*;
use super::context::*;

pub trait Handler {
    fn handle(&mut self, ctx: Context) -> Result<()>;
}
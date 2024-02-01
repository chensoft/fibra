use super::context::*;

pub trait Addon {
    async fn invoke(&mut self, _ctx: Context) {
        todo!()
    }
}
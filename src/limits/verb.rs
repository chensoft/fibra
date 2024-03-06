use crate::kernel::*;

pub trait Verb {
    fn get(&mut self) -> &mut Self;
    fn post(&mut self) -> &mut Self;
}

impl Verb for Limiter {
    fn get(&mut self) -> &mut Self {
        self.add(move |ctx| ctx.req.method() == Method::GET);
        self
    }

    fn post(&mut self) -> &mut Self {
        self.add(move |ctx| ctx.req.method() == Method::POST);
        self
    }
}
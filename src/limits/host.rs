use crate::kernel::*;

pub trait Host {
    fn host(&mut self, value: impl Into<String>) -> &mut Self;
}

impl Host for Limiter {
    fn host(&mut self, value: impl Into<String>) -> &mut Self {
        let value = value.into();
        self.add(move |ctx| ctx.req.uri().host() == Some(value.as_str()));
        self
    }
}
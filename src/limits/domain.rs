use crate::kernel::*;

pub trait Domain {
    fn domain(&mut self, value: impl Into<String>) -> &mut Self;
}

impl Domain for Limiter {
    fn domain(&mut self, value: impl Into<String>) -> &mut Self {
        let value = value.into();
        self.add(move |ctx| ctx.req.uri().host() == Some(value.as_str()));
        self
    }
}
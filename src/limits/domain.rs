use crate::kernel::*;

pub trait Domain {
    fn domain(&mut self, name: impl Into<String>);
    fn domains(&mut self, names: Vec<impl Into<String>>);
}

impl Domain for Limiter {
    fn domain(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.add(move |ctx| ctx.req.uri().host() == Some(name.as_str())); // todo
    }

    fn domains(&mut self, names: Vec<impl Into<String>>) {
        names.into_iter().for_each(|name| self.domain(name));
    }
}

impl Domain for Matcher {
    fn domain(&mut self, name: impl Into<String>) {
        todo!()
    }

    fn domains(&mut self, names: Vec<impl Into<String>>) {
        todo!()
    }
}
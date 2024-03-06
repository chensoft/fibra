use crate::kernel::*;

pub trait Path {
    fn path(&mut self, pattern: Pattern) -> &mut Self;
}

impl Path for Limiter {
    fn path(&mut self, pattern: Pattern) -> &mut Self {
        todo!()
    }
}
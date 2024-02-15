use crate::kernel::*;

pub trait Path {
    fn subdomain(&self) -> bool;
}

impl Path for Context {
    fn subdomain(&self) -> bool {
        todo!()
    }
}
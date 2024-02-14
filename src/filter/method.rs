use crate::kernel::*;

pub trait Method {
    fn is_get(&self) -> bool;
}

impl Method for Context {
    fn is_get(&self) -> bool {
        todo!()
    }
}
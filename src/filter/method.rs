use crate::kernel::*;

pub trait Method {
    fn is_get(&self) -> bool;
    fn is_post(&self) -> bool;
}

impl Method for Context {
    fn is_get(&self) -> bool {
        self.req.method() == http::Method::GET
    }

    fn is_post(&self) -> bool {
        self.req.method() == http::Method::POST
    }
}
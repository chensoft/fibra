use std::future::Future;
use crate::core::Context;

pub fn any<F, S>() -> F where F: FnMut(Context), S: Future {
    todo!()
}
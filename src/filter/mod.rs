mod func;
mod method;

pub use func::*;
pub use method::*;

use crate::Handler;

pub trait Filter: Handler {}
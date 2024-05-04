//#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]

#[macro_use] extern crate async_trait;

mod route;
mod types;

pub mod addon;

pub use route::*;
pub use types::*;
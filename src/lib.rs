// #![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::let_underscore_future)]

#[macro_use] extern crate anyhow;
#[macro_use] extern crate thiserror;
#[macro_use] extern crate async_trait;

mod macros;
mod veloce;
mod kernel;
mod limits;

pub use veloce::*;
pub use kernel::*;
pub use limits::*;

pub mod render;
pub mod addons;
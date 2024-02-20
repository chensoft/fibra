// #![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::let_underscore_future)]

#[macro_use] extern crate strum;
#[macro_use] extern crate thiserror;
#[macro_use] extern crate async_trait;

mod kernel;
mod consts;
mod macros;
mod veloce;

pub mod addons;
pub use kernel::*;
pub use consts::*;
pub use veloce::*;
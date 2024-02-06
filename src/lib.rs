// #![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::let_underscore_future)]

#[macro_use] extern crate thiserror;
#[macro_use] extern crate async_trait;

pub use http;
pub use mime;

mod kernel;
mod config;
mod consts;
mod macros;
mod traits;
mod veloce;

pub mod addons;
pub use kernel::*;
pub use config::*;
pub use consts::*;
pub use traits::*;
pub use veloce::*;
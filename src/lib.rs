// #![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]
#![allow(clippy::type_complexity)]

#[macro_use] extern crate anyhow;
#[macro_use] extern crate thiserror;
#[macro_use] extern crate async_trait;

mod veloce;
mod kernel;

pub use veloce::*;
pub use kernel::*;

pub mod addons;
pub mod render;
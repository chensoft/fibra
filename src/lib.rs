//! Fibra
//! todo
// #![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]

#[macro_use] extern crate async_trait;

mod route;
mod types;
mod fibra;

pub use route::*;
pub use types::*;
pub use fibra::*;

pub mod addon;
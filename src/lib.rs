#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]

#[macro_use] extern crate async_trait;

mod fibra;
pub use fibra::*;

pub mod addon;
pub mod route;
pub mod types;

#[doc(hidden)]
pub use route::*;
#[doc(hidden)]
pub use types::*;
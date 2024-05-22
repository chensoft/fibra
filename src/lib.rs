//! Fibra
//!
//! Fast and Powerful HTTP router written in Rust
//!
//! ## Features
//!
//! - Async-ready API
//! - Powerful router
//! - Flex middlewares
//! - Named params
//! - Glob support
//! - Regex matching
//! - URL rewrite
//! - URL redirect
//! - Domain filtering
//! - Subdomain filtering
//! - Subrouter support
//! - Stream support
//! - Varied responses
//! - Dual-stack support
//! - Multiple listeners
//!
//! ## Examples
//!
//! ```no_run
//! use fibra::*;
//!
//! #[tokio::main]
//! async fn main() -> FibraResult<()> {
//!     let mut app = Fibra::new();
//!     app.get("/", "Hello World!")?;
//!     app.bind(3000)?;
//!     app.run().await
//! }
//! ```
//!
//! **Refer to the examples folder for more use cases**
#![warn(missing_docs)]
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
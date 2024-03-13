/// Internal Use
pub(crate) use std::any::Any;
pub(crate) use std::sync::Arc;
pub(crate) use std::slice::Iter;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::collections::HashMap;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;
pub(crate) type Result<T> = anyhow::Result<T>;

/// Export Types
pub use mime::{Mime, MimeIter};
pub use hyper::{header, HeaderMap, Method, Uri, Version, body, Body, Request, Response, StatusCode};

/// Error Codes
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    StatusCode(StatusCode)
}
/// Internal Use
pub(crate) use std::pin::Pin;
pub(crate) use std::sync::Arc;
pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::collections::HashMap;
pub(crate) use std::collections::VecDeque;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;

/// Export Types
pub use mime::{Mime, MimeIter};
pub use hyper::{header, HeaderMap, Method, Uri, Version, body, Body, Request, Response, StatusCode};

/// Custom Result
pub type Result<T> = anyhow::Result<T>;

/// Error Codes
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    HostNotFound(String),

    #[error("{0}")]
    HttpStatusCode(StatusCode)
}
/// Internal Use
pub(crate) use std::any::Any;
pub(crate) use std::sync::Arc;
pub(crate) use std::slice::Iter;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;

pub(crate) use radixmap::RadixMap;

/// Export Types
pub use mime::{Mime, MimeIter};
pub use hyper::{header, HeaderMap, Method, Uri, Version, body, Body, StatusCode};

pub type Request = hyper::Request<Body>;
pub type Response = hyper::Response<Body>;

/// Error Codes
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum FibraError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    PanicError(String),

    #[error("{0}")]
    PatternError(#[from] radixmap::RadixError),

    #[error("{0}")]
    HttpError(#[from] hyper::Error),

    #[error("{0}")]
    HttpStatus(StatusCode),

    #[error("{0}")]
    HttpHeader(#[from] header::InvalidHeaderValue),
}

/// Custom Result
pub type FibraResult<T> = Result<T, FibraError>;
/// Internal Use
pub(crate) use std::any::Any;
pub(crate) use std::sync::Arc;
pub(crate) use std::slice::Iter;
pub(crate) use std::future::Future;
pub(crate) use std::net::IpAddr;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;

pub(crate) use radixmap::RadixMap;
pub(crate) use indexmap::IndexMap;
pub(crate) use chrono::{DateTime, Local};

/// Export Types
pub use mime::{Mime, MimeIter};
pub use hyper::http::{request::Parts, uri::{Scheme, Authority}};
pub use hyper::{Method, Uri, Version, header, header::{HeaderName, HeaderValue, HeaderMap}, body, Body, Request, Response, StatusCode};

/// Error Codes
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum FibraError {
    #[error("{0}")]
    PanicError(String),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("{0}")]
    PatternError(#[from] radixmap::RadixError),

    #[error("{0}")]
    HyperError(#[from] hyper::Error),

    #[error("{0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("{0}")]
    HeaderInvalid(#[from] header::InvalidHeaderValue),
}

/// Custom Result
pub type FibraResult<T> = Result<T, FibraError>;

/// Into Listener
pub trait IntoListener {
    fn into_listener(self) -> FibraResult<socket2::Socket>;
}

impl IntoListener for &str {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        match self.as_bytes().first() {
            Some(&b':') => ("0.0.0.0", self[1..].parse::<u16>()?).into_listener(),
            _ => StdTcpListener::bind(self)?.into_listener()
        }
    }
}

impl IntoListener for (&str, u16) {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        StdTcpListener::bind(self)?.into_listener()
    }
}

impl IntoListener for SocketAddr {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        StdTcpListener::bind(self)?.into_listener()
    }
}

impl IntoListener for (IpAddr, u16) {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        StdTcpListener::bind(self)?.into_listener()
    }
}

impl IntoListener for StdTcpListener {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        Ok(socket2::Socket::from(self))
    }
}

impl IntoListener for socket2::Socket {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        Ok(self)
    }
}
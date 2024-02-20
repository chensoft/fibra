/// Internal Use
pub(crate) use std::sync::Arc;
pub(crate) use std::borrow::Cow;
pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::collections::HashMap;
pub(crate) use std::collections::VecDeque;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;

/// Export Types
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, AsRefStr, EnumString)]
pub enum Method {
    GET,     // RFC 7231, 4.3.1
    HEAD,    // RFC 7231, 4.3.2
    POST,    // RFC 7231, 4.3.3
    PUT,     // RFC 7231, 4.3.4
    PATCH,   // RFC 5789
    DELETE,  // RFC 7231, 4.3.5
    CONNECT, // RFC 7231, 4.3.6
    OPTIONS, // RFC 7231, 4.3.7
    TRACE,   // RFC 7231, 4.3.8

    ANY,
    CUSTOM(String)
}

pub use mime::{Mime, MimeIter};
pub use hyper::{header, HeaderMap, Request, Response, StatusCode, Uri, Version, Body};

/// Custom Result
pub type Result<T> = anyhow::Result<T>;

/// Custom Error
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    Panicked(String),

    #[error("{0}")]
    HostNotFound(String),

    #[error("Unable to recognize {0}")]
    RouteNotFound(String),
}

// todo error use http status
// let mut res = http::Response::default();
// *res.status_mut() = match err.downcast_ref::<Error>() {
// Some(Error::RouteNotFound(_)) => http::StatusCode::NOT_FOUND,
// _ => http::StatusCode::INTERNAL_SERVER_ERROR,
// };
// res
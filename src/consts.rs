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
#[strum(serialize_all = "UPPERCASE")]
pub enum Method {
    Get,     // RFC 7231, 4.3.1
    Head,    // RFC 7231, 4.3.2
    Post,    // RFC 7231, 4.3.3
    Put,     // RFC 7231, 4.3.4
    Patch,   // RFC 5789
    Delete,  // RFC 7231, 4.3.5
    Connect, // RFC 7231, 4.3.6
    Options, // RFC 7231, 4.3.7
    Trace,   // RFC 7231, 4.3.8

    Any,
    Custom(String)
}

pub use mime::{Mime, MimeIter};
pub use hyper::{header, HeaderMap, Request, Response, StatusCode, Uri, Version, body, Body};

/// Custom Result
pub type Result<T> = anyhow::Result<T>;

/// Custom Error
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
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
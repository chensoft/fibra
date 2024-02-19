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

/// Override types
pub type Result<T> = anyhow::Result<T>;
pub type Method = Cow<'static, str>;

/// Custom errors
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
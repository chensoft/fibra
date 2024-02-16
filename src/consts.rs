pub(crate) use std::any::Any;
pub(crate) use std::sync::Arc;
pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::collections::VecDeque;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;

pub(crate) use futures::FutureExt;
pub(crate) use indexmap::IndexMap;
pub(crate) use indexmap::Equivalent;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    DNSFailed(String),

    #[error("Unable to recognize {0}")]
    NotFound(String),

    #[error("The connection was aborted")]
    Aborted,
}
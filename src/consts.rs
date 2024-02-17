pub(crate) use std::any::Any;
pub(crate) use std::sync::Arc;
pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::collections::HashMap;
pub(crate) use std::collections::VecDeque;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;

pub(crate) use futures::FutureExt;
pub(crate) use indexmap::IndexMap;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    Panicked(String),

    #[error("The connection was aborted by user")]
    Aborted,

    #[error("{0}")]
    HostNotFound(String),

    #[error("Unable to recognize {0}")]
    RouteNotFound(String),
}

// todo error handler in plugin::recover
#[derive(Debug, Clone)]
pub struct Config {
    pub catch: fn(anyhow::Error) -> http::Response<http::Body>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            catch: |err| {
                let mut res = http::Response::default();
                *res.status_mut() = match err.downcast_ref::<Error>() {
                    Some(Error::Aborted) => http::StatusCode::SERVICE_UNAVAILABLE,
                    Some(Error::RouteNotFound(_)) => http::StatusCode::NOT_FOUND,
                    _ => http::StatusCode::INTERNAL_SERVER_ERROR,
                };
                res
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Static {

}

impl Default for Static {
    fn default() -> Self {
        todo!()
    }
}
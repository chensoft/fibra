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

/// Override Result
pub type Result<T> = anyhow::Result<T>;

// /// Any support
// pub trait Any: std::any::Any {
//     /// Treat object as any
//     fn as_any(&self) -> &dyn std::any::Any;
//
//     /// Treat object as any mut
//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
// }
//
// impl<T: std::any::Any> Any for T {
//     #[inline]
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
//
//     #[inline]
//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }

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

/// App config
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
                    Some(Error::RouteNotFound(_)) => http::StatusCode::NOT_FOUND,
                    _ => http::StatusCode::INTERNAL_SERVER_ERROR,
                };
                res
            }
        }
    }
}

/// Static file config
#[derive(Debug, Clone)]
pub struct Static {

}

impl Default for Static {
    fn default() -> Self {
        todo!()
    }
}
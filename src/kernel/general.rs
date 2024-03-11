/// Internal Use
pub(crate) use std::sync::Arc;
// pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::net::ToSocketAddrs;
pub(crate) use std::marker::PhantomData;
pub(crate) use std::convert::Infallible;
pub(crate) use std::collections::HashMap;
pub(crate) use std::panic::AssertUnwindSafe;
pub(crate) use std::net::TcpListener as StdTcpListener;

/// Export Types
pub use mime::{Mime, MimeIter};
pub use hyper::{header, HeaderMap, Method, Uri, Version, body, Body, Request, Response, StatusCode};
pub use futures::future::BoxFuture;

/// Custom Result
pub type Result<T> = anyhow::Result<T>;

/// Error Codes
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    StatusCode(StatusCode)
}

/// Any support
pub trait Any: std::any::Any {
    /// Treat object as any
    fn as_any(&self) -> &dyn std::any::Any;

    /// Treat object as any mut
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: std::any::Any> Any for T {
    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
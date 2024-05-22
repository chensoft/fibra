//! Error & Result
use crate::types::*;

/// Error Codes
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum FibraError {
    #[error("{0}")]
    PanicError(Cow<'static, str>),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    TimeError(#[from] std::time::SystemTimeError),

    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("{0}")]
    RadixError(#[from] radixmap::RadixError),

    #[error("{0}")]
    HyperError(#[from] hyper::Error),

    #[error("{0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("{0}")]
    UriInvalid(#[from] hyper::http::uri::InvalidUri),

    #[error("{0}")]
    HeaderInvalid(#[from] header::InvalidHeaderValue),

    #[error("path not found")]
    PathNotFound,

    #[error("addr not available")]
    AddrNotAvailable,
}

/// Custom Result
pub type FibraResult<T> = Result<T, FibraError>;
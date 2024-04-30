//! Error & Result
use crate::types::*;

/// Error Codes
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum BoltError {
    #[error("{0}")]
    PanicError(String),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

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
pub type BoltResult<T> = Result<T, BoltError>;
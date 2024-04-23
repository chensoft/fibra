//! HTTP Headers
use crate::types::*;

/// Export
pub use hyper::header::*;

// todo Eq for HeaderValue

/// IntoHeaderValue
pub trait IntoHeaderValue {
    /// Self -> HeaderValue
    fn into_value(self) -> HeaderValue;
}

impl IntoHeaderValue for HeaderValue {
    fn into_value(self) -> HeaderValue {
        self
    }
}

impl IntoHeaderValue for Mime {
    fn into_value(self) -> HeaderValue {
        HeaderValue::try_from(self.as_ref()).unwrap_or_else(|_| unreachable!())
    }
}

impl IntoHeaderValue for &'static str {
    fn into_value(self) -> HeaderValue {
        HeaderValue::from_static(self)
    }
}
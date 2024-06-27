//! HTTP Headers
use crate::types::*;

/// Export
pub use hyper::header::*;

/// IntoHeaderName
pub trait IntoHeaderName {
    /// Self -> HeaderName
    fn into_header_name(self) -> HeaderName;
}

impl IntoHeaderName for HeaderName {
    #[inline]
    fn into_header_name(self) -> HeaderName {
        self
    }
}

impl IntoHeaderName for &'static str {
    #[inline]
    fn into_header_name(self) -> HeaderName {
        HeaderName::from_static(self)
    }
}

/// IntoHeaderValue
pub trait IntoHeaderValue {
    /// Self -> HeaderValue
    fn into_header_value(self) -> String;
}

impl IntoHeaderValue for String {
    #[inline]
    fn into_header_value(self) -> String {
        self
    }
}

impl IntoHeaderValue for Mime {
    #[inline]
    fn into_header_value(self) -> String {
        self.to_string()
    }
}

impl IntoHeaderValue for &'static str {
    #[inline]
    fn into_header_value(self) -> String {
        self.to_string()
    }
}
//! The URI component of a request
pub use hyper::Uri;

/// IntoUri
pub trait IntoUri {
    /// Self -> Uri
    fn into_uri(self) -> Uri;
}

impl IntoUri for Uri {
    #[inline]
    fn into_uri(self) -> Uri {
        self
    }
}

impl IntoUri for &'static str {
    #[inline]
    fn into_uri(self) -> Uri {
        Uri::from_static(self)
    }
}
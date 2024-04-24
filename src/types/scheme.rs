//! HTTP Scheme

/// HTTP Scheme
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Scheme {
    /// HTTP
    HTTP,

    /// HTTPS
    HTTPS,
}
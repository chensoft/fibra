//! HTTP Scheme

/// HTTP Scheme
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Scheme {
    /// HTTP
    HTTP,

    /// HTTPS
    HTTPS,
}
//! HTTP Scheme
use crate::types::*;

/// HTTP Scheme
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Scheme {
    /// HTTP
    HTTP,

    /// HTTPS
    HTTPS,
}

impl Display for Scheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::HTTP => write!(f, "http"),
            Scheme::HTTPS => write!(f, "https"),
        }
    }
}
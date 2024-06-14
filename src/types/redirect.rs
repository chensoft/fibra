//! HTTP Redirect
use crate::types::*;

/// HTTP Redirect
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Redirect {
    /// Moved Permanently
    MovedPermanently301,

    /// Found
    Found302,

    /// See Other
    SeeOther303,

    /// Temporary Redirect
    TemporaryRedirect307,

    /// Permanent Redirect
    PermanentRedirect308,
}

impl Display for Redirect {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Redirect::MovedPermanently301 => write!(f, "{}", Status::MOVED_PERMANENTLY.as_str()),
            Redirect::Found302 => write!(f, "{}", Status::FOUND.as_str()),
            Redirect::SeeOther303 => write!(f, "{}", Status::SEE_OTHER.as_str()),
            Redirect::TemporaryRedirect307 => write!(f, "{}", Status::TEMPORARY_REDIRECT.as_str()),
            Redirect::PermanentRedirect308 => write!(f, "{}", Status::PERMANENT_REDIRECT.as_str()),
        }
    }
}
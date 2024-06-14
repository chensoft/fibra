//! HTTP Status Code
use crate::types::*;

/// HTTP Status Code
pub type Status = hyper::StatusCode;

impl From<Redirect> for Status {
    fn from(value: Redirect) -> Self {
        match value {
            Redirect::MovedPermanently301 => Status::MOVED_PERMANENTLY,
            Redirect::Found302 => Status::FOUND,
            Redirect::SeeOther303 => Status::SEE_OTHER,
            Redirect::TemporaryRedirect307 => Status::TEMPORARY_REDIRECT,
            Redirect::PermanentRedirect308 => Status::PERMANENT_REDIRECT,
        }
    }
}
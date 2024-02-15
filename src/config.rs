use crate::consts::*;
use crate::kernel::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub error: fn(Context, anyhow::Error) -> http::Response<http::Body>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            error: |_ctx, err| {
                let mut resp = http::Response::default();

                *resp.status_mut() = match err.downcast::<Error>() {
                    Ok(Error::NotFound(_)) => http::StatusCode::NOT_FOUND,
                    _ => http::StatusCode::INTERNAL_SERVER_ERROR,
                };

                resp
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Static {

}

impl Default for Static {
    fn default() -> Self {
        todo!()
    }
}
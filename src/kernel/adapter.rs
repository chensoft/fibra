use crate::kernel::*;

/// Into Error
pub trait IntoError {
    fn into_error(self) -> anyhow::Error;
}

impl IntoError for StatusCode {
    fn into_error(self) -> anyhow::Error {
        anyhow!(Error::StatusCode(self))
    }
}

// todo more type like in Socket
/// Into Listener
pub trait IntoListener {
    fn into_listener(self) -> Result<socket2::Socket>;
}

impl IntoListener for &str {
    fn into_listener(self) -> Result<socket2::Socket> {
        Ok(socket2::Socket::from(StdTcpListener::bind(self)?))
    }
}

impl IntoListener for String {
    fn into_listener(self) -> Result<socket2::Socket> {
        Ok(socket2::Socket::from(StdTcpListener::bind(self)?))
    }
}

impl IntoListener for StdTcpListener {
    fn into_listener(self) -> Result<socket2::Socket> {
        Ok(socket2::Socket::from(self))
    }
}

/// Into Response
pub trait IntoResponse {
    fn into_response(self) -> Response<Body>;
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response<Body> {
        let mut res = Response::default();
        *res.status_mut() = self;
        res
    }
}

impl IntoResponse for (StatusCode, &str) {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1.to_string()).into_response()
    }
}

impl IntoResponse for (StatusCode, String) {
    fn into_response(self) -> Response<Body> {
        let mut res = Response::default();
        *res.status_mut() = self.0;
        *res.body_mut() = Body::from(self.1);
        res
    }
}
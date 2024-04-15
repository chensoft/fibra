use crate::inner::*;

/// Into Error
pub trait IntoError {
    fn into_error(self) -> FibraError;
}

impl IntoError for StatusCode {
    fn into_error(self) -> FibraError {
        FibraError::HttpStatus(self)
    }
}

/// Into Listener
pub trait IntoListener {
    fn into_listener(self) -> FibraResult<socket2::Socket>;
}

impl IntoListener for &str {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        Ok(socket2::Socket::from(StdTcpListener::bind(self)?))
    }
}

impl IntoListener for SocketAddr {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        Ok(socket2::Socket::from(StdTcpListener::bind(self)?))
    }
}

impl IntoListener for StdTcpListener {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        Ok(socket2::Socket::from(self))
    }
}

impl IntoListener for socket2::Socket {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        Ok(self)
    }
}

/// Into Response
pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        let mut res = Response::default();
        *res.status_mut() = self;
        res
    }
}

impl IntoResponse for (StatusCode, &'static str) {
    fn into_response(self) -> Response {
        let mut res = Response::default();
        *res.status_mut() = self.0;
        *res.body_mut() = Body::from(self.1);
        res
    }
}

impl IntoResponse for (StatusCode, String) {
    fn into_response(self) -> Response {
        let mut res = Response::default();
        *res.status_mut() = self.0;
        *res.body_mut() = Body::from(self.1);
        res
    }
}
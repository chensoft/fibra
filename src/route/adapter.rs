use crate::types::*;

/// Into Error
pub trait IntoError {
    fn into_error(self) -> FibraError;
}

impl IntoError for StatusCode {
    fn into_error(self) -> FibraError {
        FibraError::StatusCode(self)
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
    fn into_response(self) -> Response<Body>;
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self)
            .body(Body::default())
            .unwrap_or_else(|_| unreachable!())
    }
}

impl<T> IntoResponse for (StatusCode, T)
    where
        T: Into<Body>,
{
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.0)
            .body(self.1.into())
            .unwrap_or_else(|_| unreachable!())
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .body(self.into())
            .unwrap_or_else(|_| unreachable!())
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .body(self.into())
            .unwrap_or_else(|_| unreachable!())
    }
}
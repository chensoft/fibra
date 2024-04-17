use crate::types::*;

/// Into Listener
pub trait IntoListener {
    fn into_listener(self) -> FibraResult<socket2::Socket>;
}

impl IntoListener for &str {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        match self.as_bytes().first() {
            Some(&b':') => ("0.0.0.0", self[1..].parse::<u16>()?).into_listener(),
            _ => StdTcpListener::bind(self)?.into_listener()
        }
    }
}

impl IntoListener for (&str, u16) {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        StdTcpListener::bind(self)?.into_listener()
    }
}

impl IntoListener for SocketAddr {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        StdTcpListener::bind(self)?.into_listener()
    }
}

impl IntoListener for (IpAddr, u16) {
    fn into_listener(self) -> FibraResult<socket2::Socket> {
        StdTcpListener::bind(self)?.into_listener()
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

impl<T> IntoResponse for (StatusCode, Mime, T)
    where
        T: Into<Body>,
{
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.0)
            .header(header::CONTENT_TYPE.as_str(), self.1.as_ref())
            .body(self.2.into())
            .unwrap_or_else(|_| unreachable!())
    }
}

impl<T> IntoResponse for (StatusCode, T)
    where
        T: Into<Body>,
{
    fn into_response(self) -> Response<Body> {
        (self.0, mime::TEXT_PLAIN, self.1).into_response()
    }
}

impl IntoResponse for () {
    fn into_response(self) -> Response<Body> {
        (StatusCode::OK, mime::TEXT_PLAIN, Body::empty()).into_response()
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response<Body> {
        (self, mime::TEXT_PLAIN, Body::empty()).into_response()
    }
}

impl IntoResponse for Body {
    fn into_response(self) -> Response<Body> {
        (StatusCode::OK, mime::TEXT_PLAIN, self).into_response()
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response<Body> {
        (StatusCode::OK, mime::TEXT_PLAIN, self).into_response()
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response<Body> {
        (StatusCode::OK, mime::TEXT_PLAIN, self).into_response()
    }
}

impl IntoResponse for Vec<u8> {
    fn into_response(self) -> Response<Body> {
        (StatusCode::OK, mime::APPLICATION_OCTET_STREAM, self).into_response()
    }
}

impl IntoResponse for &'static [u8] {
    fn into_response(self) -> Response<Body> {
        (StatusCode::OK, mime::APPLICATION_OCTET_STREAM, self).into_response()
    }
}
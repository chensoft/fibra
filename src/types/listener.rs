//! Address to Listener
use crate::types::*;
use std::net::ToSocketAddrs;
use socket2::{Domain, Type, Protocol};

/// Address to Listener
pub trait TryIntoListener {
    /// Self -> Listener
    fn try_into_listener(self) -> FibraResult<Socket>;
}

impl TryIntoListener for u16 {
    fn try_into_listener(self) -> FibraResult<Socket> {
        if let Ok(socket) = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], self)).try_into_listener() {
            return Ok(socket);
        }

        if let Ok(socket) = SocketAddr::from(([0, 0, 0, 0], self)).try_into_listener() {
            return Ok(socket);
        }

        Err(FibraError::AddrNotAvailable)
    }
}

/// Supports multiple string formats
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// assert_eq!(":3000".try_into_listener().is_ok(), true);           // dual-stack, v4 & v6
/// assert_eq!("[::]:3000".try_into_listener().is_ok(), true);       // dual-stack, v4 & v6
/// assert_eq!("[::1]:3000".try_into_listener().is_ok(), true);      // ipv6-only
/// assert_eq!("0.0.0.0:3000".try_into_listener().is_ok(), true);    // ipv4-only
/// assert_eq!("0.0.0.0:65536".try_into_listener().is_ok(), false);  // invalid port
/// assert_eq!("0.0.0.256:3000".try_into_listener().is_ok(), false); // invalid addr
/// assert_eq!("[::1]:65536".try_into_listener().is_ok(), false);    // invalid addr
/// assert_eq!("[::256]:3000".try_into_listener().is_ok(), false);   // invalid addr
/// ```
impl TryIntoListener for &str {
    fn try_into_listener(self) -> FibraResult<Socket> {
        if self.as_bytes().first() == Some(&b':') {
            return self[1..].parse::<u16>()?.try_into_listener();
        }

        for address in self.to_socket_addrs()? {
            if let Ok(socket) = address.try_into_listener() {
                return Ok(socket);
            }
        }

        Err(FibraError::AddrNotAvailable)
    }
}

impl TryIntoListener for String {
    fn try_into_listener(self) -> FibraResult<Socket> {
        self.as_str().try_into_listener()
    }
}

impl TryIntoListener for SocketAddr {
    fn try_into_listener(self) -> FibraResult<Socket> {
        let domain = match &self {
            SocketAddr::V4(_) => Domain::IPV4,
            SocketAddr::V6(_) => Domain::IPV6,
        };

        let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
        socket.set_reuse_address(true)?;
        socket.bind(&self.into())?;
        socket.listen(128)?;

        Ok(socket)
    }
}

impl TryIntoListener for StdTcpListener {
    fn try_into_listener(self) -> FibraResult<Socket> {
        Ok(Socket::from(self))
    }
}

impl TryIntoListener for Socket {
    fn try_into_listener(self) -> FibraResult<Socket> {
        Ok(self)
    }
}

impl TryIntoListener for std::io::Result<Socket> {
    fn try_into_listener(self) -> FibraResult<Socket> {
        Ok(self?)
    }
}
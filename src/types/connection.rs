//! HTTP Connection
use crate::types::*;

/// This object represents a single HTTP connection. A client can send multiple requests on a single
/// connection if using HTTP/1x's Keep-Alive feature or HTTP/2.
pub struct Connection {
    /// The time the connection was created
    created: SystemTime,

    /// The count of requests processed
    count: AtomicUsize,

    /// The endpoint on the local machine for the connection
    sockaddr: SocketAddr,

    /// The remote address that the connection comes from
    peeraddr: SocketAddr,
}

impl Connection {
    /// Create a new object
    #[inline]
    pub fn new() -> Self {
        Self::from((([0, 0, 0, 0], 0), ([0, 0, 0, 0], 0)))
    }

    /// Get the created time of this connection
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// assert_eq!(Connection::new().created_ref() <= &SystemTime::now(), true);
    /// ```
    #[inline]
    pub fn created_ref(&self) -> &SystemTime {
        &self.created
    }

    /// Get/Set the created time of this connection
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// let now = SystemTime::now();
    /// let mut con = Connection::new();
    /// *con.created_mut() = now;
    ///
    /// assert_eq!(con.created_ref(), &now);
    /// ```
    #[inline]
    pub fn created_mut(&mut self) -> &mut SystemTime {
        &mut self.created
    }

    /// Set the created time of this connection
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::time::SystemTime;
    ///
    /// let now = SystemTime::now();
    /// let con = Connection::new().created(now);
    ///
    /// assert_eq!(con.created_ref(), &now);
    /// ```
    #[inline]
    pub fn created(mut self, val: impl Into<SystemTime>) -> Self {
        self.created = val.into();
        self
    }

    /// Get the count of requests processed
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    ///
    /// assert_eq!(Connection::new().count_ref().load(Ordering::Relaxed), 0);
    /// ```
    #[inline]
    pub fn count_ref(&self) -> &AtomicUsize {
        &self.count
    }

    /// Increase the count of requests
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// assert_eq!(Connection::new().count_add(0), 0);
    /// assert_eq!(Connection::new().count_add(5), 5);
    /// ```
    #[inline]
    pub fn count_add(&self, incr: usize) -> usize {
        self.count.fetch_add(incr, atomic::Ordering::Relaxed) + incr
    }

    /// Get/Set the count of requests processed
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    ///
    /// let mut con = Connection::new();
    /// *con.count_mut() = AtomicUsize::new(12345);
    ///
    /// assert_eq!(con.count_ref().load(Ordering::Relaxed), 12345);
    /// ```
    #[inline]
    pub fn count_mut(&mut self) -> &mut AtomicUsize {
        &mut self.count
    }

    /// Set the count of requests processed
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::sync::atomic::{Ordering};
    ///
    /// assert_eq!(Connection::new().count(12345).count_ref().load(Ordering::Relaxed), 12345);
    /// ```
    #[inline]
    pub fn count(mut self, val: usize) -> Self {
        self.count = AtomicUsize::new(val);
        self
    }

    /// Get the local address
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::new().sockaddr_ref(), &SocketAddr::from(([0, 0, 0, 0], 0)));
    /// ```
    #[inline]
    pub fn sockaddr_ref(&self) -> &SocketAddr {
        &self.sockaddr
    }

    /// Get/Set the local address
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// let mut con = Connection::new();
    /// *con.sockaddr_mut() = SocketAddr::from(([127, 0, 0, 1], 3000));
    ///
    /// assert_eq!(con.sockaddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    #[inline]
    pub fn sockaddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.sockaddr
    }

    /// Set the local address
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::new().sockaddr(([127, 0, 0, 1], 3000)).sockaddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    #[inline]
    pub fn sockaddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.sockaddr = val.into();
        self
    }

    /// Get the remote address
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::new().peeraddr_ref(), &SocketAddr::from(([0, 0, 0, 0], 0)));
    /// ```
    #[inline]
    pub fn peeraddr_ref(&self) -> &SocketAddr {
        &self.peeraddr
    }

    /// Get/Set the remote address
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// let mut con = Connection::new();
    /// *con.peeraddr_mut() = SocketAddr::from(([127, 0, 0, 1], 3000));
    ///
    /// assert_eq!(con.peeraddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    #[inline]
    pub fn peeraddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.peeraddr
    }

    /// Set the remote address
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::new().peeraddr(([127, 0, 0, 1], 3000)).peeraddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    #[inline]
    pub fn peeraddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.peeraddr = val.into();
        self
    }
}

impl Default for Connection {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Into<SocketAddr>, P: Into<SocketAddr>> From<(S, P)> for Connection {
    #[inline]
    fn from((sock, peer): (S, P)) -> Self {
        Self { count: 0.into(), created: SystemTime::now(), sockaddr: sock.into(), peeraddr: peer.into() }
    }
}
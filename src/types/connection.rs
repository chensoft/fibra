//! HTTP Connection
use crate::types::*;

/// This object represents a single HTTP connection. A client can send multiple requests on a single
/// connection if using HTTP/1.1's keep-alive feature or HTTP/2.
pub struct Connection {
    /// The unique identifier of this connection
    id: u128,

    /// The count of requests processed
    count: u64,

    /// The time the connection was created
    created: DateTime<Local>,

    /// The local address that is connected
    sockaddr: SocketAddr,

    /// The remote address that the connection comes from or connects to
    peeraddr: SocketAddr,
}

impl Connection {
    /// Get the unique identifier of this connection
    ///
    /// ```
    /// use fibra::{Connection};
    ///
    /// assert_eq!(*Connection::default().id_ref() > 0, true);
    /// ```
    pub fn id_ref(&self) -> &u128 {
        &self.id
    }

    /// Get/Set the unique identifier of this connection
    ///
    /// ```
    /// use fibra::{Connection};
    ///
    /// let mut con = Connection::default();
    /// *con.id_mut() = 12345;
    ///
    /// assert_eq!(con.id_ref(), &12345);
    /// ```
    pub fn id_mut(&mut self) -> &mut u128 {
        &mut self.id
    }

    /// Set the unique identifier of this connection
    ///
    /// ```
    /// use fibra::{Connection};
    ///
    /// assert_eq!(Connection::default().id(12345).id_ref(), &12345);
    /// ```
    pub fn id(mut self, val: u128) -> Self {
        self.id = val;
        self
    }

    /// Get the count of requests processed
    ///
    /// ```
    /// use fibra::{Connection};
    ///
    /// assert_eq!(*Connection::default().count_ref(), 0);
    /// ```
    pub fn count_ref(&self) -> &u64 {
        &self.count
    }

    /// Get/Set the count of requests processed
    ///
    /// ```
    /// use fibra::{Connection};
    ///
    /// let mut con = Connection::default();
    /// *con.count_mut() = 12345;
    ///
    /// assert_eq!(con.count_ref(), &12345);
    /// ```
    pub fn count_mut(&mut self) -> &mut u64 {
        &mut self.count
    }

    /// Set the count of requests processed
    ///
    /// ```
    /// use fibra::{Connection};
    ///
    /// assert_eq!(Connection::default().count(12345).count_ref(), &12345);
    /// ```
    pub fn count(mut self, val: u64) -> Self {
        self.count = val;
        self
    }

    /// Get the created time of this connection
    ///
    /// ```
    /// use chrono::Local;
    /// use fibra::{Connection};
    ///
    /// assert_eq!(Connection::default().created_ref() <= &Local::now(), true);
    /// ```
    pub fn created_ref(&self) -> &DateTime<Local> {
        &self.created
    }

    /// Get/Set the created time of this connection
    ///
    /// ```
    /// use chrono::Local;
    /// use fibra::{Connection};
    ///
    /// let now = Local::now();
    /// let mut con = Connection::default();
    /// *con.created_mut() = now;
    ///
    /// assert_eq!(con.created_ref(), &now);
    /// ```
    pub fn created_mut(&mut self) -> &mut DateTime<Local> {
        &mut self.created
    }

    /// Set the created time of this connection
    ///
    /// ```
    /// use chrono::Local;
    /// use fibra::{Connection};
    ///
    /// let now = Local::now();
    /// let con = Connection::default().created(now);
    ///
    /// assert_eq!(con.created_ref(), &now);
    /// ```
    pub fn created(mut self, val: DateTime<Local>) -> Self {
        self.created = val;
        self
    }

    /// Get the local address
    ///
    /// ```
    /// use fibra::{Connection};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::default().sockaddr_ref(), &SocketAddr::from(([0, 0, 0, 0], 0)));
    /// ```
    pub fn sockaddr_ref(&self) -> &SocketAddr {
        &self.sockaddr
    }

    /// Get/Set the local address
    ///
    /// ```
    /// use fibra::{Connection};
    /// use std::net::SocketAddr;
    ///
    /// let mut con = Connection::default();
    /// *con.sockaddr_mut() = SocketAddr::from(([127, 0, 0, 1], 3000));
    ///
    /// assert_eq!(con.sockaddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn sockaddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.sockaddr
    }

    /// Set the local address
    ///
    /// ```
    /// use fibra::{Connection};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::default().sockaddr(([127, 0, 0, 1], 3000)).sockaddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn sockaddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.sockaddr = val.into();
        self
    }

    /// Get the remote address
    ///
    /// ```
    /// use fibra::{Connection};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::default().peeraddr_ref(), &SocketAddr::from(([0, 0, 0, 0], 0)));
    /// ```
    pub fn peeraddr_ref(&self) -> &SocketAddr {
        &self.peeraddr
    }

    /// Get/Set the remote address
    ///
    /// ```
    /// use fibra::{Connection};
    /// use std::net::SocketAddr;
    ///
    /// let mut con = Connection::default();
    /// *con.peeraddr_mut() = SocketAddr::from(([127, 0, 0, 1], 3000));
    ///
    /// assert_eq!(con.peeraddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn peeraddr_mut(&mut self) -> &mut SocketAddr {
        &mut self.peeraddr
    }

    /// Set the remote address
    ///
    /// ```
    /// use fibra::{Connection};
    /// use std::net::SocketAddr;
    ///
    /// assert_eq!(Connection::default().peeraddr(([127, 0, 0, 1], 3000)).peeraddr_ref(), &SocketAddr::from(([127, 0, 0, 1], 3000)));
    /// ```
    pub fn peeraddr(mut self, val: impl Into<SocketAddr>) -> Self {
        self.peeraddr = val.into();
        self
    }
}

/// Default trait
impl Default for Connection {
    fn default() -> Self {
        Self::from((([0, 0, 0, 0], 0), ([0, 0, 0, 0], 0)))
    }
}

/// Create a new connection
impl<S: Into<SocketAddr>, P: Into<SocketAddr>> From<(S, P)> for Connection {
    fn from((sock, peer): (S, P)) -> Self {
        Self { id: Ulid::new().0, count: 0, created: Local::now(), sockaddr: sock.into(), peeraddr: peer.into() }
    }
}
use crate::consts::*;

pub struct Address {
    pub local: SocketAddr,
    pub remote: SocketAddr,
}

impl Address {
    pub fn new(local: SocketAddr, remote: SocketAddr) -> Self {
        Self {local, remote}
    }
}
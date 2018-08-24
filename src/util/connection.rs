use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

/// An abstraction over various connection types.
pub trait Connection: AsyncRead + AsyncWrite {
    /// Attempts to retrieve a peer's socket address of the connection.
    ///
    /// A default implementation simply returns `None`.
    fn peer_addr(&self) -> Option<SocketAddr> {
        None
    }
}

impl Connection for TcpStream {
    fn peer_addr(&self) -> Option<SocketAddr> {
        match self.peer_addr() {
            Ok(addr) => Some(addr),
            Err(e) => {
                error!("Can't get a peer's address: {}", e);
                None
            }
        }
    }
}

use socket2::{Domain, SockAddr, Socket, Type};
use std::net::SocketAddr;
use tokio::io::Result;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct Receiver {
    socket: UdpSocket,
}

impl Receiver {
    pub async fn new(bind_addr: &str) -> Result<Self> {
        // Parse the bind address into a standard SocketAddr
        let addr: SocketAddr = bind_addr.parse().expect("Unable to parse socket address");

        // Create the socket using socket2 to set options
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;
        socket.set_reuse_address(true)?;
        socket.bind(&SockAddr::from(addr))?;

        // Convert the socket2 socket to a tokio socket
        let std_socket = std::net::UdpSocket::from(socket);
        std_socket.set_nonblocking(true)?;
        let socket = UdpSocket::from_std(std_socket)?;

        Ok(Receiver { socket })
    }

    pub async fn receive<F>(&self, callback: F) -> Result<()>
    where
        F: Fn(String) + Send + 'static,
    {
        let mut buf = vec![0; 1024];
        loop {
            let (len, _) = self.socket.recv_from(&mut buf).await?;
            let msg = String::from_utf8_lossy(&buf[..len]).to_string();
            callback(msg);
        }
    }
}

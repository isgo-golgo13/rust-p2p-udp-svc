use tokio::io::Result;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct Sender {
    socket: UdpSocket,
}

impl Sender {
    pub async fn new(bind_addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        Ok(Sender { socket })
    }

    pub async fn send(&self, msg: &str, target_addr: &str) -> Result<()> {
        self.socket.send_to(msg.as_bytes(), target_addr).await?;
        Ok(())
    }
}

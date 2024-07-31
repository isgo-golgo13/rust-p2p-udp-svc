use dotenv::dotenv;
use endpoints::receiver::Receiver;
use endpoints::sender::Sender;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[derive(Clone, Debug)]
struct P2P {
    sender: Arc<Mutex<Sender>>,
    receiver: Arc<Mutex<Receiver>>,
}

impl P2P {
    async fn new(send_addr: &str, recv_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sender = Sender::new(send_addr).await?;
        let receiver = Receiver::new(recv_addr).await?;
        Ok(P2P {
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
        })
    }

    async fn send(&self, msg: &str, target_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sender = self.sender.lock().await;
        sender.send(msg, target_addr).await?;
        println!("Sent: {}", msg);
        Ok(())
    }

    async fn receive<F>(&self, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(String) + Send + 'static,
    {
        let receiver = self.receiver.lock().await;
        receiver.receive(callback).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let node_addr = env::var("NODE_ADDR").expect("NODE_ADDR not set");
    let peer_addr = env::var("PEER_ADDR").expect("PEER_ADDR not set");

    let p2p = match P2P::new(&node_addr, &node_addr).await {
        Ok(p2p) => p2p,
        Err(e) => {
            eprintln!("Failed to create P2P instance: {:?}", e);
            return;
        }
    };

    let p2p_clone = p2p.clone();
    task::spawn(async move {
        if let Err(e) = p2p_clone
            .receive(|msg| {
                println!("Received: {}", msg);
            })
            .await
        {
            eprintln!("Failed to receive messages: {:?}", e);
        }
    });

    loop {
        if let Err(e) = p2p.send("Hello from Rust P2P!", &peer_addr).await {
            eprintln!("Failed to send message: {:?}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use serde_json;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub struct Node {
    blockchain: Arc<Mutex<Blockchain>>,
    peers: Arc<Mutex<HashSet<String>>>,
    address: String,
}

impl Node {
    pub fn new(address: String) -> Self {
        Node {
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
            peers: Arc::new(Mutex::new(HashSet::new())),
            address,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Node listening on {}", self.address);

        loop {
            let (mut socket, _) = listener.accept().await?;
            let blockchain = Arc::clone(&self.blockchain);
            let peers = Arc::clone(&self.peers);
            let address = self.address.clone();
            tokio::spawn(async move {
                Self::handle_connection(socket, blockchain, peers, address).await;
            });
        }
    }

    async fn handle_connection(
        mut socket: TcpStream,
        blockchain: Arc<Mutex<Blockchain>>,
        peers: Arc<Mutex<HashSet<String>>>,
        address: String,
    ) {
        let mut buffer = [0; 1024];
        let n = socket.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer[..n]);
        println!("Received message: {}", message);

        let response = match message.trim() {
            _ if message.starts_with("NEW_TRANSACTION:") => {
                let tx_json = &message[16..];
                let transaction: Transaction = serde_json::from_str(tx_json).unwrap();
                let tx_hash = transaction.calculate_hash();
                println!("Transaction hash: {:?}", tx_hash);
                blockchain.lock().await.add_transaction(transaction);
                "Transaction added".to_string()
            }
            _ if message.starts_with("GET_BLOCK_TO_MINE") => {
                let block_to_mine = blockchain.lock().await.get_block_to_mine();
                match block_to_mine {
                    Some(block) => serde_json::to_string(&block).unwrap(),
                    None => "NO_BLOCK_AVAILABLE".to_string(),
                }
            }
            _ if message.starts_with("MINED_BLOCK:") => {
                let block: Block = serde_json::from_str(&message[12..]).unwrap();
                println!("Received mined block with nonce: {}", block.get_nonce());
                blockchain.lock().await.add_mined_block(block);
                "Mined block added to blockchain".to_string()
            }
            _ if message.starts_with("ADD_PEER:") => {
                let new_peer = message[9..].to_string();
                peers.lock().await.insert(new_peer.clone());
                format!("Peer {} added", new_peer)
            }
            "GET_PEERS" => {
                let peer_list = peers
                    .lock()
                    .await
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(",");
                format!("PEER_LIST:{}", peer_list)
            }
            _ => "Unknown command".to_string(),
        };

        println!("Sending response: {}", response);
        socket.write_all(response.as_bytes()).await.unwrap();
    }

    pub async fn broadcast_transaction(&self, transaction: &Transaction) {
        let tx_json = serde_json::to_string(&transaction).unwrap();
        let message = format!("NEW_TRANSACTION:{}", tx_json);

        for peer in self.peers.lock().await.iter() {
            if let Ok(mut stream) = TcpStream::connect(peer).await {
                stream.write_all(message.as_bytes()).await.unwrap();
            }
        }
    }

    pub async fn add_peer(&self, address: String) {
        self.peers.lock().await.insert(address.clone());
        let message = format!("ADD_PEER:{}", self.address);
        if let Ok(mut stream) = TcpStream::connect(&address).await {
            stream.write_all(message.as_bytes()).await.unwrap();
        }
    }

    pub async fn discover_peers(
        &self,
        bootstrap_node: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(bootstrap_node).await?;
        stream.write_all(b"GET_PEERS").await?;

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);

        if response.starts_with("PEER_LIST:") {
            let peer_list = response[10..].split(',');
            for peer in peer_list {
                if peer != self.address {
                    self.add_peer(peer.to_string()).await;
                }
            }
        }

        Ok(())
    }
}
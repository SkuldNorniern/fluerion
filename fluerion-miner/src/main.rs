use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, BufRead, Write};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use num_cpus;

#[derive(serde::Deserialize, serde::Serialize,Debug,Clone)]
struct Block {
    timestamp: u64,
    prev_block_hash: [u8; 32],
    hash: [u8; 32],
    transactions: Vec<Transaction>,
    nonce: u64,
}

impl Block {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(serde::Deserialize, serde::Serialize,Debug,Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    timestamp: u64,
    signature: Option<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Fluerion Miner");

    let mut address = String::new();
    print!("Enter your miner address: ");
    io::stdout().flush()?;
    io::stdin().lock().read_line(&mut address)?;
    let address = address.trim().to_string();

    let mut node_addr = String::new();
    print!("Enter the node address to connect to (e.g., 127.0.0.1:8080): ");
    io::stdout().flush()?;
    io::stdin().lock().read_line(&mut node_addr)?;
    let node_addr = SocketAddr::from_str(node_addr.trim()).expect("Invalid address");

    println!("Connecting to node at {}", node_addr);

    let mut stream = TcpStream::connect(node_addr).await?;
    println!("Miner {} connected to node", address);

    loop {
        println!("Waiting for block to mine...");
        match receive_block(&mut stream).await? {
            Some(block) => {
                println!("Received block to mine");
                let mined_block = mine_block_multi_threaded(block);
                println!("Block mined successfully!");
                send_mined_block(&mut stream, &mined_block).await?;
                println!("Mined block sent to node");
            }
            None => {
                println!("No block available to mine. Waiting for 5 seconds...");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

async fn receive_block(stream: &mut TcpStream) -> io::Result<Option<Block>> {
    println!("Sending request to get block to mine...");
    stream.write_all(b"GET_BLOCK_TO_MINE").await?;
    println!("Request sent.");

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Received response: {}", response);

    if response.trim().is_empty() {
        Ok(None)
    } else if response.starts_with("NO_BLOCK_AVAILABLE") {
        Ok(None)
    } else {
        match serde_json::from_str(&response) {
            Ok(block) => Ok(Some(block)),
            Err(e) => {
                println!("Error parsing block data: {}", e);
                Ok(None)
            }
        }
    }
}

fn mine_block_multi_threaded(mut block: Block) -> Block {
    let mut target = [0xFF; 32];
    target[0] = 0x00; // Adjust this value to set the desired difficulty
    let block = Arc::new(Mutex::new(block));
    let found = Arc::new(Mutex::new(false));
    let num_threads = num_cpus::get();

    let start_time = Instant::now();
    let progress = Arc::new(Mutex::new(0u64));

    let threads: Vec<_> = (0..num_threads)
        .map(|i| {
            let block = Arc::clone(&block);
            let found = Arc::clone(&found);
            let progress = Arc::clone(&progress);

            thread::spawn(move || {
                let mut local_block = block.lock().unwrap().clone();
                let mut nonce = i as u64;
                let mut local_progress = 0u64;

                while !*found.lock().unwrap() {
                    local_block.nonce = nonce;
                    let hash = calculate_hash(&local_block);
                    if hash < target {
                        let mut block = block.lock().unwrap();
                        *block = local_block;
                        block.hash = hash;
                        *found.lock().unwrap() = true;
                        println!("Nonce found: {}", nonce);
                        break;
                    }
                    nonce += num_threads as u64;
                    local_progress += 1;

                    // if local_progress % 10_000 == 0 {
                        // println!("Thread {}: Nonce {} - Local progress: {} hashes", i, nonce, local_progress);
                    // }

                    if local_progress % 100_000 == 0 {
                        *progress.lock().unwrap() += local_progress;
                        local_progress = 0;
                    }
                }
            })
        })
        .collect();

    let progress_thread = thread::spawn(move || {
        while !*found.lock().unwrap() {
            thread::sleep(Duration::from_secs(5));
            let total_progress = *progress.lock().unwrap();
            let elapsed = start_time.elapsed().as_secs();
            let hash_rate = if elapsed > 0 { total_progress / elapsed } else { 0 };
            let estimated_time = if hash_rate > 0 {
                let target_value = u64::from_be_bytes(target[0..8].try_into().unwrap());
                let max_nonce = u64::MAX;
                (max_nonce - target_value) / hash_rate
            } else {
                0
            };
            println!("Mining progress: {} hashes/s, Estimated time: {} seconds", hash_rate, estimated_time);
        }
    });

    for thread in threads {
        thread.join().unwrap();
    }

    progress_thread.join().unwrap();

    Arc::try_unwrap(block).unwrap().into_inner().unwrap()
}

fn calculate_hash(block: &Block) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&block.timestamp.to_le_bytes());
    hasher.update(&block.prev_block_hash);
    hasher.update(&block.nonce.to_le_bytes());
    
    // Include transactions data
    for tx in &block.transactions {
        hasher.update(tx.sender.as_bytes());
        hasher.update(tx.receiver.as_bytes());
        hasher.update(&tx.amount.to_le_bytes());
        hasher.update(&tx.timestamp.to_le_bytes());
    }
    
    let result = hasher.finalize();
    let mut hash = [0; 32];
    hash.copy_from_slice(&result);
    hash
}

async fn send_mined_block(stream: &mut TcpStream, block: &Block) -> io::Result<()> {
    let block_json = block.to_json();
    let message = format!("MINED_BLOCK:{}", block_json);
    stream.write_all(message.as_bytes()).await?;
    Ok(())
}
use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, BufRead, Write};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    timestamp: u64,
    signature: Option<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Fluerion Wallet");

    let mut address = String::new();
    print!("Enter your wallet address: ");
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
    println!("Wallet {} connected to node", address);

    loop {
        let mut choice = String::new();
        println!("\n1. Send transaction");
        println!("2. Check balance");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush()?;
        io::stdin().lock().read_line(&mut choice)?;

        match choice.trim() {
            "1" => send_transaction(&mut stream, &address).await?,
            "2" => check_balance(&mut stream, &address).await?,
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }

    Ok(())
}

async fn send_transaction(stream: &mut TcpStream, sender: &str) -> io::Result<()> {
    let mut receiver = String::new();
    print!("Enter receiver address: ");
    io::stdout().flush()?;
    io::stdin().lock().read_line(&mut receiver)?;
    let receiver = receiver.trim().to_string();

    let mut amount = String::new();
    print!("Enter amount: ");
    io::stdout().flush()?;
    io::stdin().lock().read_line(&mut amount)?;
    let amount: f64 = amount.trim().parse().expect("Invalid amount");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let transaction = Transaction {
        sender: sender.to_string(),
        receiver,
        amount,
        timestamp,
        signature: None,
    };

    let tx_json = serde_json::to_string(&transaction).unwrap();
    let message = format!("NEW_TRANSACTION:{}", tx_json);

    stream.write_all(message.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Response: {}", response);

    Ok(())
}

async fn check_balance(stream: &mut TcpStream, address: &str) -> io::Result<()> {
    let message = format!("GET_BALANCE:{}", address);
    stream.write_all(message.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Balance: {}", response);

    Ok(())
}
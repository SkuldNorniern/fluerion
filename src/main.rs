mod block;
mod blockchain;
mod hash;
mod network;
mod transaction;

use network::Node;
use std::env;
use transaction::Transaction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <node_address> [bootstrap_node]", args[0]);
        std::process::exit(1);
    }

    let node_address = args[1].clone();
    let mut node = Node::new(node_address.clone());

    if args.len() > 2 {
        let bootstrap_node = &args[2];
        println!("Discovering peers from bootstrap node: {}", bootstrap_node);
        node.discover_peers(bootstrap_node).await?;
    }

    // Create and broadcast a transaction
    let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
    node.broadcast_transaction(&transaction).await;

    // Start the node
    node.start().await?;

    Ok(())
}

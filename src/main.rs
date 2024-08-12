mod block;
mod hash;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Transaction 1".to_string());
    blockchain.add_block("Transaction 2".to_string());
    blockchain.add_block("Transaction 3".to_string());

    println!("Is blockchain valid? {}", blockchain.is_valid());

    // Print the latest block's hash
    println!("Latest block hash: {:?}", blockchain.get_latest_block().get_hash());
}

mod block;
mod hash;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0));
    blockchain.add_transaction(Transaction::new("Bob".to_string(), "Charlie".to_string(), 30.0));
    blockchain.mine_pending_transactions();

    blockchain.add_transaction(Transaction::new("Charlie".to_string(), "David".to_string(), 20.0));
    blockchain.add_transaction(Transaction::new("David".to_string(), "Alice".to_string(), 15.0));
    blockchain.mine_pending_transactions();

    println!("Is blockchain valid? {}", blockchain.is_valid());

    println!("\nBlockchain contents:");
    blockchain.print_chain();
}
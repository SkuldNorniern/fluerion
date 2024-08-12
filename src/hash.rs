use sha2::{Sha512, Digest};

pub type Hash256 = [u8; 32];

pub fn calculate_hash(timestamp: &str, prev_block_hash: &Hash256, data: &str) -> Hash256 {
    let mut hasher = Sha512::new();

    // Update the hasher with all the input data
    hasher.update(timestamp.as_bytes());
    hasher.update(prev_block_hash);
    hasher.update(data.as_bytes());

    // Finalize and truncate to 256 bits
    let result = hasher.finalize();
    let mut hash256 = [0u8; 32];
    hash256.copy_from_slice(&result[..32]);

    hash256
}

pub fn hash_to_hex(hash: &Hash256) -> String {
    hash.iter().map(|byte| format!("{:02x}", byte)).collect()
}
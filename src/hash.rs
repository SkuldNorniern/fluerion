pub type Hash256 = [u8; 32];

pub fn calculate_hash(timestamp: &str, prev_block_hash: &Hash256, data: &str) -> Hash256 {
    let mut result = [0u8; 32];
    let mut index = 0;

    // Simple custom hashing algorithm (not cryptographically secure, just for demonstration)
    for byte in timestamp.bytes().chain(prev_block_hash.iter().cloned()).chain(data.bytes()) {
        result[index % 32] ^= byte;
        result[(index + 1) % 32] = result[(index + 1) % 32].wrapping_add(byte);
        result[(index + 2) % 32] = result[(index + 2) % 32].wrapping_sub(byte);
        index += 3;
    }

    result
}

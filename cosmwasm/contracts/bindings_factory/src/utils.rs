use sha2::{Sha256, Digest};
use cosmwasm_std::Binary;
use hex::encode as hex_encode; // Use the `hex` crate for conversion

/// Hashes a given message using SHA-256 and returns the result as a `[u8; 32]` byte array.
pub fn hash_msg(msg: &Binary) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(msg.as_slice());
    hasher.finalize().into()
}

pub fn hash_to_hex(hash: [u8; 32]) -> String {
    hex_encode(hash) // Converts bytes to a lowercase hex string
}

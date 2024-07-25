use cosmos_sdk::tendermint::Error;
use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo},
    Coin,
    bip32,

};

use thiserror::Error;

use std::{panic, str::{self, FromStr}};

/// jackal chain id
const CHAIN_ID: &str = "puppy-1";

/// RPC port
const RPC_PORT: u16 = 26657;

/// Expected account number
const ACCOUNT_NUMBER: AccountNumber = 1;

/// Bech32 prefix for an account
const ACCOUNT_PREFIX: &str = "jkl";

/// Denom name
const DENOM: &str = "ujkl";

/// Example memo
const MEMO: &str = "test memo";

struct SafeSigningKeyDisplay {
    key: cosmrs::crypto::secp256k1::SigningKey,
}

impl std::fmt::Debug for SafeSigningKeyDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SigningKey")
         .field("key", &"***sensitive information hidden***")
         .finish()
    }
}

fn main() {

    env_logger::init();

    let seed = "fork draw talk diagram fragile online style lecture ecology lawn dress hat";

    let path_str = "m/44'/0'/0'/0/0"; // placeholder
    let path = parse_derivation_path(path_str).expect("Failed to parse derivation path");

        // Usage in your match statement
    match secp256k1::SigningKey::derive_from_path(seed, &path) {
        Ok(key) => {
            let safe_key_display = SafeSigningKeyDisplay { key };
            log::info!("Sender private key derived successfully: {:?}", safe_key_display);
        },
        Err(e) => {
            log::error!("Failed to derive sender private key: {}", e);
        }
}

    println!("Hello, world!");
}

// Assuming the necessary imports and module structure
fn parse_derivation_path(path_str: &str) -> Result<bip32::DerivationPath, bip32::Error> {
    let path = bip32::DerivationPath::from_str(path_str)?;
    Ok(path)
}


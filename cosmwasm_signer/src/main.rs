use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo},
    Coin,
};

use std::{panic, str};

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

fn main() {

    
    println!("Hello, world!");
}
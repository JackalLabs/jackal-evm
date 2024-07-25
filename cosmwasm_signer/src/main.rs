extern crate bip39;
extern crate hex;

use cosmos_sdk::tendermint::Error;
use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo},
    Coin,
    bip32::{XPrv, DerivationPath, Seed, Mnemonic}, // using the bip32 that was re-imported by cosmrs
};

use thiserror::Error;
use std::{panic, str::{self, FromStr}};
use bip39::{Language};
use hex::ToHex;

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

/// Devnet seed phrase
const SEED: &str = "fork draw talk diagram fragile online style lecture ecology lawn dress hat modify member leg pluck leaf depend subway grit trumpet tongue crucial stumble";

fn main() {

    let mnemonic = Mnemonic::new(SEED, Default::default()).unwrap();
    let seed = mnemonic.to_seed(""); 
    let root_xprv = XPrv::new(&seed).expect("failed to get root xprv");


    let child_path = "m/44'/118'/0'/0/0";
    let sender_private_key = secp256k1::SigningKey::derive_from_path(seed, &child_path.parse().expect("no child_path")).expect("failed to generate private key");
    let child_xpub = sender_private_key.public_key();

    let signing_key = sender_private_key;
    let verification_key = child_xpub;

    let sender_account_id = verification_key.account_id("jkl").expect("failed to get account");
    
    println!("{}", sender_account_id);

}

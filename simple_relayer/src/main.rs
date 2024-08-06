//! The relayer forwards signed checkpoints from the current chain's mailbox to 
//! the other chains' mailboxes
//! 
//! At a regular interval, the relayer polls the current chain's mailbox for 
//! signed checkpoints and submits them as checkpoints on the remote mailbox.

use tokio::time::{interval, Duration};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use anyhow::Result;

use cosmos_sdk::tendermint::Error;
use cosmrs::{
    bank::MsgSend, 
    bip32::{secp256k1::Secp256k1, DerivationPath, Mnemonic, Seed, XPrv}, 
    cosmwasm::MsgExecuteContract, 
    crypto::{secp256k1, PublicKey}, 
    crypto::secp256k1::SigningKey,
    rpc, 
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo}, AccountId, Coin
};

use cosmwasm_std::{Addr, Binary};
use thiserror::Error;
use std::{panic, str::{self, FromStr}};
use bip39::{Language};
use hex::ToHex;
use tendermint_rpc::{Client, HttpClient, endpoint::abci_query::AbciQuery, Response};
use tendermint_rpc::endpoint::status::Response as StatusResponse;
use tendermint::chain::Id;

/// RPC port
const RPC_PORT: u16 = 59610; // provided by docker image of canine-chain booted up by interchaintest suite

/// Denom name
const DENOM: &str = "ujkl";

/// Expected account number
const ACCOUNT_NUMBER: AccountNumber = 12;

/// Example memo
const MEMO: &str = "test memo";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Load seed phrase from environment
    let seed_phrase = env::var("SEED_PHRASE")?;

    let mut account_id: Option<AccountId> = None;
    let mut private_key: Option<SigningKey> = None;

    match generate_account_id_from_seed(seed_phrase) {
        Ok((id, key)) => {
            account_id = Some(id);
            private_key = Some(key);
            println!("Generated account ID: {:?}", account_id);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        },
    }

    let private_key = private_key.expect("Private key not found");

    // Create a shared Cosmos client
    let rpc_address = format!("http://localhost:{}", RPC_PORT);
    let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap(); // this is from cosmrs-rpc (which is just imported tendermint-rpc)

    // Spawn a task for sending tokens periodically
    tokio::spawn(start_token_sender(rpc_client.clone(), account_id.expect("no account_id found
    ").clone(), private_key.public_key(), private_key));

    // Keep the main thread alive
    loop {
        tokio::signal::ctrl_c().await?;
        println!("Received Ctrl+C, shutting down.");
        break;
    }

    Ok(())
}

async fn start_token_sender(client: rpc::HttpClient, account: AccountId, public_key: PublicKey, signing_key: secp256k1::SigningKey) -> Result<()> {
    let mut interval = interval(Duration::from_secs(5));
    let mut sequence_number = 5;

    loop {
        interval.tick().await;

        // create the transaction
        let amount = Coin {
            amount: 50u8.into(), 
            denom: DENOM.parse().unwrap(),
        };

        let msg_send = MsgSend {
            from_address: account.clone(),
            to_address: AccountId::from_str("jkl19gwxcq9646e4ndgxldr6ygqx0562ul5frpvt00").unwrap(), // just a random address
            amount: vec![amount.clone()],
        }
        .to_any()
        .unwrap();

        let chain_id_str = "puppy-1";
        let chain_id = Id::from_str(chain_id_str).expect("Failed to create chain ID");
    
        // on fresh run, expected sequence is 3 
        // NOTE: This sequence number might need to be declared outside the loop?
        let used_sequence = sequence_number + 1; // TODO: in prod, will need to query for this for the specific account, and increment it 
        let gas = 500_000u64;
        let fee = Fee::from_amount_and_gas(amount, gas);

        let tx_body = tx::BodyBuilder::new().msg(msg_send).memo(MEMO).finish();
        let auth_info =
        SignerInfo::single_direct(Some(public_key), used_sequence).auth_info(fee);

        // sign the transaction
        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, ACCOUNT_NUMBER).unwrap();
        let tx_raw = sign_doc.sign(&signing_key).unwrap();

        // broadcast the transaction 
        let tx_commit_response = tx_raw.broadcast_commit(&client).await.unwrap();

        if tx_commit_response.check_tx.code.is_err() {
            panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
        }

        if tx_commit_response.tx_result.code.is_err() {
            panic!("tx_result error: {:?}", tx_commit_response.tx_result);
        }

        let tx_hash = tx_commit_response.hash;
        println!("{}", tx_hash);

        // increase the sequence number if the tx succeeds
        sequence_number += 1;

    }
}

fn generate_account_id_from_seed(seed_phrase: String) -> Result<(AccountId, secp256k1::SigningKey), Box<dyn std::error::Error>> {
    // Generate mnemonic
    let mnemonic = Mnemonic::new(&seed_phrase, Default::default())?;
    
    // Convert mnemonic to seed
    let seed = mnemonic.to_seed(""); 

    // Define the derivation path
    let child_path = "m/44'/118'/0'/0/0";
    
    // Derive the sender's private key
    let sender_private_key = secp256k1::SigningKey::derive_from_path(seed, &child_path.parse()?)?;
    
    // Get the corresponding public key
    let public_key = sender_private_key.public_key();

    // Generate the sender's account ID
    let sender_account_id = public_key.account_id("jkl")?;

    Ok((sender_account_id, sender_private_key))
}
//! The relayer forwards signed checkpoints from the current chain's mailbox to 
//! the other chains' mailboxes
//! 
//! At a regular interval, the relayer polls the current chain's mailbox for 
//! signed checkpoints and submits them as checkpoints on the remote mailbox.

use tokio::{sync::mpsc, time::{interval, Duration}};
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
use filetree::msg::ExecuteMsg;

use web3::contract::{Contract, Options};
use web3::futures::StreamExt;
use web3::transports::{Http, WebSocket};
use web3::types::{FilterBuilder, H160, Log};
use web3::{ethabi, Web3};
use ethabi::{decode, ParamType};

/// RPC port
const RPC_PORT: u16 = 55710; // provided by docker image of canine-chain booted up by interchaintest suite

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
    let mut sequence_number: u64 = 5;

    // filetreeContractAddr := "jkl1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq59a839"

    // Set up WebSocket connection and event listener for EVM
    let web3_socket = Web3::new(WebSocket::new("ws://localhost:8545").await?);
    
    // Channel to receive the event data from the listener
    let (event_tx, mut event_rx) = mpsc::channel::<String>(10);

    let address_str = "0x5fbdb2315678afecb367f032d93f642f64180aa3";
    let address = H160::from_str(address_str).expect("Invalid address format");

    // Spawn the event listener task
    tokio::spawn(async move {
        let contract_event_data_listener = create_event_data_listener(&web3_socket, address, event_tx.clone()).await?;
        contract_event_data_listener.await.map_err(|e| web3::Error::from(e.to_string()))?;
        Ok::<(), web3::Error>(())
    });

    loop {
        interval.tick().await;

        let amount = Coin {
            amount: 50u8.into(), 
            denom: DENOM.parse().unwrap(),
        };

        // Receive the event value from the channel
        let event_value = match event_rx.recv().await {
        Some(value) => value,
        None => {
            eprintln!("Failed to receive event value");
            continue; // Skip this iteration if no event value is received
        }
    };

    // Use the received event value in the key_value string
    let key_value = format!("{} with sequence: {}", event_value, sequence_number);

            
        let msg = ExecuteMsg::PostKey { key: key_value };
        let json_msg = serde_json::to_string(&msg).unwrap();
        let json_msg_binary = Binary::from(json_msg.into_bytes());
    
        // execute CosmWasm mailbox
        let mailbox_execute_msg = MsgExecuteContract {
            sender: AccountId::from_str("jkl12g4qwenvpzqeakavx5adqkw203s629tf6k8vdg").unwrap(),
            contract: AccountId::from_str("jkl1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq59a839").unwrap(),
            msg: json_msg_binary.to_vec(), // just json
            funds: [].to_vec(),  
        }.to_any().unwrap();

        let chain_id_str = "puppy-1";
        let chain_id = Id::from_str(chain_id_str).expect("Failed to create chain ID");
    
        // on fresh run, expected sequence is 3 
        // NOTE: This sequence number might need to be declared outside the loop?
        let used_sequence = sequence_number + 1; // TODO: in prod, will need to query for this for the specific account, and increment it 
        let gas = 500_000u64;
        let fee = Fee::from_amount_and_gas(amount, gas);

        let tx_body = tx::BodyBuilder::new().msg(mailbox_execute_msg).memo(MEMO).finish();
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

async fn create_event_data_listener(
    web3_socket: &Web3<WebSocket>, 
    address: H160, 
    event_tx: mpsc::Sender<String>
) -> web3::Result<tokio::task::JoinHandle<()>> {
    let filter = FilterBuilder::default()
        .address(vec![address])
        .build();

    let sub = Web3::eth_subscribe(web3_socket).subscribe_logs(filter).await?;

    Ok(tokio::spawn(async move {
        sub.for_each(|log| async {
            match log {
                Ok(log) => {
                    if let Some(value) = decode_event_data(&log) {
                        // Send the event value through the channel
                        if let Err(e) = event_tx.send(value).await {
                            eprintln!("Failed to send event value: {:?}", e);
                        }
                    } else {
                        println!("Failed to decode event value");
                    }
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }).await;
    }))
}



fn decode_event_data(log: &Log) -> Option<String> {
    // Decode only the 'value' part of the log data (skip 'sender' and 'message')
    let decoded_data = decode(
        &[ParamType::String], // Only decode the 'string value' part
        &log.data.0,
    ).ok()?;

    // Extract and return the 'value' string
    let value = decoded_data[0].clone().to_string();
    Some(value)
}
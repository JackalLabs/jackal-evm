use std::str::FromStr;

use tokio::{sync::mpsc, time::{interval, Duration}};
use tokio::task;
use web3::transports::WebSocket;
use web3::Web3;
use web3::types::{FilterBuilder, H160, Log};

use crate::config::Config;
use crate::signer;

use cosmrs::{
    bank::MsgSend, 
    bip32::{secp256k1::Secp256k1, DerivationPath, Mnemonic, Seed, XPrv}, 
    cosmwasm::MsgExecuteContract, 
    crypto::{secp256k1, PublicKey}, 
    crypto::secp256k1::SigningKey,
    rpc, 
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo}, AccountId, Coin
};

use filetree::msg::ExecuteMsg;
use cosmwasm_std::{Addr, Binary};
use tendermint::chain::Id;
use web3::{ethabi};
use ethabi::{decode, ParamType};
use anyhow::Result;
use web3::futures::StreamExt;
use crate::network::create_event_data_listener;
use crate::query::query_account;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use crate::queue::BoundedQueue;

/// Denom name
const DENOM: &str = "ujkl";

/// Expected account number
const ACCOUNT_NUMBER: AccountNumber = 12;

/// Example memo
const MEMO: &str = "test memo";

pub struct Relayer {
    config: Config,
}

impl Relayer {

    pub fn new(config: Config) -> Self {
        Relayer { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        // TODO: NOTE - This might be a good method for rigorously testing the program.
        // Every single piece of data received from the channel will be its own unique filetree entry
        // On canine-chain, we back up the entries to a json file so we can confirm that the channel actually works
        // A lot of manual labour here but building out a truely complete integration test would take much longer

        // We sign with our private key 
        let (account_id, signing_key) = signer::generate_account_id_from_seed(&self.config.cosmos_seed_phrase)?;

        println!("{:?}", account_id);

        let rpc_address = &self.config.cosmos_rpc_url;
        let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap(); // this is from cosmrs-rpc (which is just imported tendermint-rpc)
        

        // Spawn a task for sending tokens periodically
        tokio::spawn(start_token_sender(self.config.evm_contract_address.clone(),rpc_client.clone(), account_id.clone(), signing_key.public_key(), signing_key));

    // Keep the main thread alive
    loop {
        tokio::signal::ctrl_c().await?;
        println!("Received Ctrl+C, shutting down.");
        break;
    }

        Ok(())
    }
}

async fn start_token_sender(evm_contract_address: String, client: rpc::HttpClient, account: AccountId, public_key: PublicKey, signing_key: secp256k1::SigningKey) -> Result<()> {
    // TODO: make this shorter
    let mut interval = interval(Duration::from_secs(5));

    // Define the address and gRPC URL
    let address = "jkl12g4qwenvpzqeakavx5adqkw203s629tf6k8vdg".to_string(); // Replace with the actual address
    let grpc_url = "http://localhost:50456".to_string(); // Replace with the actual gRPC URL

    let mut sequence_number: u64 = 0;

    let rest_client: Client = Client::new();
    // TODO: gotta take this from config
    let url = "http://localhost:53220/cosmos/auth/v1beta1/accounts/jkl12g4qwenvpzqeakavx5adqkw203s629tf6k8vdg";

            // Call the function and handle the result
        match query_account(&rest_client, url).await {
            Ok(account_response) => {
                sequence_number = account_response.account.sequence.parse::<u64>()?; 
            },
            Err(e) => {
                eprintln!("Failed to get account sequence number: {}", e);
                return Err(e.into()); // Exit early if there's an error
            }
        }

    // Set up WebSocket connection and event listener for EVM
    let web3_socket = Web3::new(WebSocket::new("ws://localhost:8545").await?);
    
    // Channel to receive the event data from the listener
    let (event_tx, mut event_rx) = mpsc::channel::<String>(10);

    // Convert evm contract address to H60 
    let address = H160::from_str(&evm_contract_address).expect("Invalid address format");

    // TODO: Rigorously test to make sure it doesn't need a larger capacity 
    // Could also set the capacity to 10000?
    // Add resource monitoring

    // Bounded queue with a capacity of 1000
    let bounded_queue = Arc::new(Mutex::new(BoundedQueue::new(1000)));

    // Receive data from the event listener and enqueueing it

    {
        let bounded_queue = bounded_queue.clone(); // TODO: double check this is memory efficient
        tokio::spawn(async move {
            let contract_event_data_listener =
                create_event_data_listener(&web3_socket, address, event_tx.clone()).await?;
            contract_event_data_listener.await.map_err(|e| web3::Error::from(e.to_string()))?;
            Ok::<(), web3::Error>(())
        });
    }



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
    println!("event value: {}", event_value);

            
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

    }
}

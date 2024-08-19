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

        // We sign with our private key 
        let (account_id, signing_key) = signer::generate_account_id_from_seed(&self.config.cosmos_seed_phrase)?;

        println!("{:?}", account_id);

        let rpc_address = &self.config.cosmos_rpc_url;
        let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap(); // this is from cosmrs-rpc (which is just imported tendermint-rpc)
        

        // Spawn a task for sending tokens periodically
        tokio::spawn(start_token_sender(rpc_client.clone(), account_id.clone(), signing_key.public_key(), signing_key));

    // Keep the main thread alive
    loop {
        tokio::signal::ctrl_c().await?;
        println!("Received Ctrl+C, shutting down.");
        break;
    }

        Ok(())
    }
}

async fn start_token_sender(client: rpc::HttpClient, account: AccountId, public_key: PublicKey, signing_key: secp256k1::SigningKey) -> Result<()> {
    let mut interval = interval(Duration::from_secs(5));
    let mut sequence_number: u64 = 185; // NOTE: DO NOW - really need to query for this number

    // Set up WebSocket connection and event listener for EVM
    let web3_socket = Web3::new(WebSocket::new("ws://localhost:8545").await?);
    
    // Channel to receive the event data from the listener
    let (event_tx, mut event_rx) = mpsc::channel::<String>(10);

    let address_str = "0xbc71f5687cfd36f64ae6b4549186ee3a6ee259a4";
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

        // increase the sequence number if the tx succeeds
        sequence_number += 1;

    }
}

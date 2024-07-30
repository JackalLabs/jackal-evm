extern crate bip39;
extern crate hex;

use cosmos_sdk::tendermint::Error;
use cosmrs::{
    bank::MsgSend,
    cosmwasm::MsgExecuteContract,
    crypto::secp256k1,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo},
    Coin,
    bip32::{XPrv, DerivationPath, Seed, Mnemonic}, // using the bip32 that was re-imported by cosmrs
    rpc,
    AccountId,
};

use cosmwasm_std::{Addr, Binary};
use thiserror::Error;
use std::{panic, str::{self, FromStr}};
use bip39::{Language};
use hex::ToHex;
use tendermint_rpc::{Client, HttpClient, endpoint::abci_query::AbciQuery, Response};
use tendermint_rpc::endpoint::status::Response as StatusResponse;
use tendermint::chain::Id;

use mailbox::msg::ExecuteMsg;

/// jackal chain id
const CHAIN_ID: &str = "puppy-1";

/// RPC port
const RPC_PORT: u16 = 55140; // provided by docker image of canine-chain booted up by interchaintest suite

/// Expected account number
const ACCOUNT_NUMBER: AccountNumber = 12;

/// Bech32 prefix for an account
const ACCOUNT_PREFIX: &str = "jkl";

/// Denom name
const DENOM: &str = "ujkl";

/// Example memo
const MEMO: &str = "test memo";

/// Devnet seed phrase
const SEED: &str = "brief enhance flee chest rabbit matter chaos clever lady enable luggage arrange hint quarter change float embark canoe chalk husband legal dignity music web";
fn main() {

    let mnemonic = Mnemonic::new(SEED, Default::default()).unwrap();
    let seed = mnemonic.to_seed(""); 
    let root_xprv = XPrv::new(&seed).expect("failed to get root xprv");


    let child_path = "m/44'/118'/0'/0/0";
    let sender_private_key = secp256k1::SigningKey::derive_from_path(seed, &child_path.parse().expect("no child_path")).expect("failed to generate private key");
    let child_xpub = sender_private_key.public_key();

    let verification_key = child_xpub;
    let sender_account_id = verification_key.account_id("jkl").expect("failed to get account");
    
    println!("{}", sender_account_id);

    // if we send funds to this account, they should come into existence?

    let recipient_private_key = secp256k1::SigningKey::random();
    let recipient_account_id = recipient_private_key
        .public_key()
        .account_id(ACCOUNT_PREFIX)
        .unwrap();

    println!("{}", recipient_account_id);

    let amount = Coin {
        amount: 68u8.into(), 
        denom: DENOM.parse().unwrap(),
    };

    let msg_send = MsgSend {
        from_address: sender_account_id.clone(),
        to_address: recipient_account_id,
        amount: vec![amount.clone()],
    }
    .to_any()
    .unwrap();

    let msg = ExecuteMsg::Signer {};
    let json_msg = serde_json::to_string(&msg).unwrap();
    let json_msg_binary = Binary::from(json_msg.into_bytes());

    // TODO: deploy mailbox to local docker before calling the below
    // execute CosmWasm mailbox
    let mailbox_execute_msg = MsgExecuteContract {
        sender: AccountId::from_str("jkl12g4qwenvpzqeakavx5adqkw203s629tf6k8vdg").unwrap(),
        contract: AccountId::from_str("jkl14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9scsc9nr").unwrap(),
        msg: json_msg_binary.to_vec(), // just json
        funds: [].to_vec(),  
    }.to_any().unwrap();

    let chain_id_str = "puppy-1";
    let chain_id = Id::from_str(chain_id_str).expect("Failed to create chain ID");

    // on fresh run, expected sequence is 3 
    let sequence_number = 7; // TODO: in prod, will need to query for this for the specific account, and increment it 
    let gas = 500_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    let tx_body = tx::BodyBuilder::new().msg(mailbox_execute_msg).memo(MEMO).finish();
    let auth_info =
        SignerInfo::single_direct(Some(verification_key), sequence_number).auth_info(fee);
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, ACCOUNT_NUMBER).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    init_tokio_runtime().block_on(async {
        let rpc_address = format!("http://localhost:{}", RPC_PORT); // the container's 26657 port will bind to another port on your machine 
        // let rpc_client = HttpClient::new(rpc_address.as_str()).unwrap(); // this was from tendermint-rpc 
        let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap(); // this is from cosmrs-rpc (which is just imported tendermint-rpc)
        let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

        if tx_commit_response.check_tx.code.is_err() {
            panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
        }

        if tx_commit_response.tx_result.code.is_err() {
            panic!("tx_result error: {:?}", tx_commit_response.tx_result);
        }

        let tx_hash = tx_commit_response.hash;
        println!("{}", tx_hash)
    })

}

/// Initialize Tokio runtime
fn init_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
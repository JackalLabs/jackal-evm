use web3::contract::{Contract, Options};
use web3::futures::StreamExt;
use web3::transports::{Http, WebSocket};
use web3::types::{FilterBuilder, H160, Log};
use web3::{ethabi, Web3};
use ethabi::{decode, ParamType};

async fn create_event_data_listener(web3_socket: &Web3<WebSocket>, address: H160) -> web3::Result<tokio::task::JoinHandle<()>> {
    let filter = FilterBuilder::default()
        .address(vec![address])
        .build();

    let sub = Web3::eth_subscribe(web3_socket).subscribe_logs(filter).await?;

    Ok(tokio::spawn(async move {
        sub.for_each(|log| async{
            match log {
                Ok(log) => {
                    if let Some(value) = decode_event_data(&log) {
                        println!("Value received: {}", value);
                    } else {
                        println!("Failed to decode event value");
                    }
                }
                Err(e) => eprintln!("Error: {:?}", e)
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


async fn deploy_test_contract(web3: Web3<Http>) -> web3::Result<Contract<Http>> {
    let abi = include_bytes!("build/MockEvent.abi");
    let bytecode = include_str!("build/MockEvent.bin");
    
    let accounts = web3.eth().accounts().await?;

    let contract = Contract::deploy(web3.eth(), abi)
        .map_err(|e| web3::Error::from(e.to_string()))?
        .confirmations(1)
        .options(Options::with(|opt| {
            opt.gas = Some(3_000_000.into());
        }))
        .execute(bytecode, (), accounts[0])
        .await
        .map_err(|e| web3::Error::from(e.to_string()))?;

    Ok(contract)
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let web3_http = Web3::new(Http::new("http://localhost:8545")?);
    let accounts = web3_http.eth().accounts().await?;
    let contract = deploy_test_contract(web3_http).await.expect("Could not deploy test contract!");
    let contract_address = contract.address();
    println!("Contract deployed at address: {:?}", contract_address);
    
    let web3_socket = Web3::new(WebSocket::new("ws://localhost:8545").await?);
    let contract_event_data_listener = create_event_data_listener(&web3_socket, contract_address).await?;

    contract.call("dispatchEvent", ("Hello, World!".to_string(),), accounts[0], Options::default()).await.map_err(|e| web3::Error::from(e.to_string()))?;
    contract_event_data_listener.await.map_err(|e| web3::Error::from(e.to_string()))?;
    Ok(())
}
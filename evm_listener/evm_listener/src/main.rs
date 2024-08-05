use web3::contract::{Contract, Options};
use web3::futures::StreamExt;
use web3::transports::{Http, WebSocket};
use web3::types::{Address, FilterBuilder, H160};
use web3::Web3;

async fn listen_contract_events(web3_socket: &Web3<WebSocket>, address: H160) -> web3::Result<()> {
    let filter = FilterBuilder::default()
        .address(vec![address])
        .build();

    let sub = Web3::eth_subscribe(web3_socket).subscribe_logs(filter).await?;

    sub.for_each(|log| async{
        match log {
            Ok(log) => {
                println!("Got log: {:?}", log);
            }
            Err(e) => eprintln!("Error: {:?}", e)
        }
    }).await;

    Ok(())
}

async fn deploy_mailbox(web3: Web3<Http>) -> web3::Result<Address> {
    let abi = include_bytes!("build/TestEvent.abi");
    let bytecode = include_str!("build/TestEvent.bin");
    
    let accounts = web3.eth().accounts().await?;

    let contract = Contract::deploy(web3.eth(), abi)
        .map_err(|e| web3::Error::from(e.to_string()))?
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.gas = Some(3_000_000.into());
        }))
        .execute(bytecode, (), accounts[0])
        .await
        .map_err(|e| web3::Error::from(e.to_string()))?;

    Ok(contract.address())
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let web3_http = Web3::new(Http::new("http://localhost:8545")?);
    let contract_address = deploy_mailbox(web3_http).await.expect("Could not deploy and run dispatch event!");
    
    let web3_socket = Web3::new(WebSocket::new("ws://localhost:8545").await?);
    listen_contract_events(&web3_socket, contract_address).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    
}
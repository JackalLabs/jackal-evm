use std::any::Any;

use web3::futures::{ future, StreamExt };
use web3::transports::WebSocket;
use web3::types::BlockId;
use web3::Web3;


/// Return the first contract that gets added to the chain after the function is called
async fn get_next_contract_address(web3: Web3<WebSocket>) -> Result<(), web3::Error> {
    // Subscribe to new block headers
    let mut sub = web3.eth_subscribe().subscribe_new_heads().await?;

    // Continuously receive new block headers
    while let Some(blockheader_result) = sub.next().await {
        let block = blockheader_result?;
        let block_hash = block.hash.unwrap();
        let block_transactions = web3.eth().block_with_txs(BlockId::from(block_hash)).await?.unwrap().transactions;
        

        for tx in block_transactions {
            let receipt = web3.eth().transaction_receipt(tx.hash).await?;
            if let Some(receipt) = receipt {
                println!("{:?}", receipt);
                let contract_address = receipt.contract_address;
                if let Some(contract_address) = contract_address {
                    println!("Contract Address! {:?}", contract_address);
                }
            }
        }

        future::ready(()).await;
    }

    sub.unsubscribe().await?;
    Ok(())
}
    // Get the address of the Mailbox contract on the EVM chain and the signature of the Dispatch event
/*    let mail_evm_addr = "";
    let dispatch_sig= web3::signing::keccak256(b"Dispatch(address,uint32,bytes32,bytes)");
*/

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Connect to local evm chain
    let transport = WebSocket::new("ws://localhost:8545").await?;
    let web3 = Web3::new(transport);
    let whatever = get_next_contract_address(web3).await;
    // Close the application and return Ok
    Ok(())
}
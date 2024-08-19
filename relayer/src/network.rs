use tokio::{sync::mpsc};
use web3::transports::WebSocket;
use web3::Web3;
use web3::types::{FilterBuilder, H160, Log};

use web3::{ethabi};
use ethabi::{decode, ParamType};
use anyhow::Result;
use web3::futures::StreamExt;

pub(crate) fn decode_event_data(log: &Log) -> Option<String> {
    // Decode only the 'value' part of the log data (skip 'sender' and 'message')
    let decoded_data = decode(
        &[ParamType::String], // Only decode the 'string value' part
        &log.data.0,
    ).ok()?;

    // Extract and return the 'value' string
    let value = decoded_data[0].clone().to_string();
    Some(value)
}

pub(crate) async fn create_event_data_listener(
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
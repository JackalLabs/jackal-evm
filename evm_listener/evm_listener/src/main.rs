use web3::futures::{ future, StreamExt };
use web3::transports::WebSocket;
use web3::Web3;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = WebSocket::new("ws://localhost:8545").await?;
    let web3 = Web3::new(transport);

    let mut sub = web3.eth_subscribe().subscribe_new_heads().await?;

    (&mut sub)
        .take(5)
        .for_each(|x| {
            println!("Got: {:?}", x);
            future::ready(())
        })
        .await;

    sub.unsubscribe().await?;
    Ok(())
}
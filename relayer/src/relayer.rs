use tokio::sync::mpsc;
use tokio::task;
use web3::transports::WebSocket;

use crate::config::Config;
use crate::signer;

pub struct Relayer {
    config: Config,
}

impl Relayer {

    pub fn new(config: Config) -> Self {
        Relayer { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        let (account_id, signing_key) = signer::generate_account_id_from_seed(&self.config.cosmos_seed_phrase)?;

        println!("{:?}", account_id);

        // // Channel to receive the event data from the listener
        // let (event_tx, mut event_rx) = mpsc::channel::<String>(10);

        // let ws = WebSocket::new(&self.config.evm_websocket_url).await?;
        // let cosmos_rpc_url = self.config.cosmos_rpc_url.clone();

        // // spawn task
        Ok(())
    }
}
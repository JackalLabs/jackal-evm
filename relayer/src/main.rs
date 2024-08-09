mod config;
mod relayer;
mod signer;

use config::Config;
use relayer::Relayer;
use log::info;
use std::path::Path;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config.toml");
    let config = Config::from_file(config_path.to_str().unwrap())
        .expect("Failed to load config");

    println!("{}", config.cosmos_rpc_url);
    println!("{}", config.cosmos_seed_phrase);
    println!("{}", config.evm_websocket_url);

    info!("Starting relayer service...");

    let relayer = Relayer::new(config);
    relayer.run().await.expect("Relayer service failed");
}
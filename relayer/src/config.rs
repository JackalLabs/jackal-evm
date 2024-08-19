use serde::Deserialize;

// TODO: I think it's best to put the private key in config.toml, not the seed phrase?
// Either put the private key here, or take in the seed phrase as an ENV var, generate the private key, 
// then save it to memory and forget it when service is stopped?
#[derive(Deserialize)]
pub struct Config {
    pub cosmos_seed_phrase: String,
    pub evm_websocket_url: String,
    pub cosmos_rpc_url: String,
    pub evm_contract_address: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
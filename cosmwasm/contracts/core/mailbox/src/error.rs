use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("invalid config. reason: {reason:?}")]
    InvalidConfig { reason: String },

    #[error("invalid message version: {version:?}")]
    InvalidMessageVersion { version: u8 },

    #[error("invalid destination domain: {domain:?}")]
    InvalidDestinationDomain { domain: u32 },

    #[error("message already delivered")]
    AlreadyDeliveredMessage {},

    #[error("ism verify failed")]
    VerifyFailed {},
}

impl ContractError {
    pub fn invalid_config(reason: &str) -> Self {
        Self::InvalidConfig {
            reason: reason.to_string(),
        }
    }
}

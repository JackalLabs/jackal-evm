use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Bindings contract already created for this user. Bindings Contract Address: {0}")]
    AlreadyCreated(String),

    #[error("This user does not have a bindings contract")]
    DoesNotExist(),
}



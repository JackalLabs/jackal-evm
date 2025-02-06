use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Only white listed addresses can call bindings")]
    NotAllowed(),

    #[error("Only the factory owner can update the white list")]
    CannotUpdate(),
}



use cosmwasm_schema::cw_serde;
use hpl_interface::core::mailbox::ExecuteMsg as ExternalExecuteMsg;
use cosmwasm_std::{DepsMut, Env, HexBinary, MessageInfo, Response, StdError, StdResult};
use serde::{Serialize, Deserialize};

// NOTE: we mistakenly previously tried to add a 'signer' variant to the hpl_interface::ExecuteMsg variant
// and this is why our compiler was not recognizing the 'signer' variant.
// To fix this, we now properly wrap ExecuteMsg from hpl_interface
// Might be worth just copying it out in the future for ease of integration with jackal.js and other TS APIs 
#[cw_serde]
pub enum ExecuteMsg {
    External(ExternalExecuteMsg),
    Signer {},
}

impl From<ExternalExecuteMsg> for ExecuteMsg {
    fn from(msg: ExternalExecuteMsg) -> Self {
        ExecuteMsg::External(msg)
    }
}

impl From<ExecuteMsg> for StdResult<ExternalExecuteMsg> {
    fn from(msg: ExecuteMsg) -> Self {
        match msg {
            ExecuteMsg::External(external_msg) => Ok(external_msg),
            ExecuteMsg::Signer {} => Err(StdError::generic_err("Cannot convert Signer to external ExecuteMsg")),
        }
    }
}
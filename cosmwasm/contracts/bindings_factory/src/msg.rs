use cosmwasm_schema::{cw_serde, QueryResponses};
use shared::shared_msg::SharedExecuteMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub bindings_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {

    CallBindings {
        evm_address: String, // Will use this to find mapped bindings contract address to call 
        msg:         SharedExecuteMsg, // Just raw JSON
    },
    AddToWhiteList {
        jkl_address: String, 
    },

}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// GetContractState returns the contact's state.
    #[returns(crate::state::ContractState)]
    GetContractState {},
    /// GetUserBindingsAddress returns the bindings contract address owned by the given user address
    #[returns(String)]
    GetUserBindingsAddress { user_address: String},
    /// GetAllUserBindingsAddresses returns all user bindings addresses in a readable format
    #[returns(Vec<(String, String)>)]
    GetAllUserBindingsAddresses {},
    /// GetWhiteList returns the white list
    #[returns(Vec<(String, bool)>)]
    GetWhiteList {},
}


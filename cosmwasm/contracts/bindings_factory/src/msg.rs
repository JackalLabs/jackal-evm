use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub bindings_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {

    CreateBindings {
        // Not really sure what I need right now 
    },

    MapUserBindings {
        // Not really sure what I need right now 
    }

    CallBindings {
        evm_address: String, // Will use this to find mapped bindings contract address to call 
        msg:         String, // Just raw JSON? 
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
}

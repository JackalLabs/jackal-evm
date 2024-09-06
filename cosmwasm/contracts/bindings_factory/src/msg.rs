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

    CreateBindingsV2 {
        user_evm_address: String,
        // This makes use of CosmWasm's 'instantiate2' API, which pre computes the contract address.
        // If it works on canine-chain, will save us a lot of code
    },

    MapUserBindings {
        // Not really sure what I need right now 
    },

    CallBindings {
        evm_address: String, // Will use this to find mapped bindings contract address to call 
        msg:         FiletreeExecuteMsg, // Just raw JSON? 
    },

}

// Linker gets confused if we import filetree's msg types, so we can just our own copy with a different name
#[cw_serde]
pub enum FiletreeExecuteMsg {

    PostKey {
        key: String,
    },

    MakeRoot {
        editors: String,
        viewers: String,
        trackingnumber: String,
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

}

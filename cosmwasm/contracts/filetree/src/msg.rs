use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {

    PostKey {
        key: String,
    },

    MakeRoot {
        editors: String,
        viewers: String,
        trackingnumber: String,
    },
    
}

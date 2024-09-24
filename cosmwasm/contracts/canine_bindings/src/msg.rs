use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {

    PostKey {
        key: String,
    },

    PostFile {
        merkle: String,
        file_size: i64,
        proof_interval: i64,
        proof_type: i64,
        max_proofs: i64,
        expires: i64,
        note: String,
    },

    MakeRoot {
        editors: String,
        viewers: String,
        trackingnumber: String,
    },
    
}

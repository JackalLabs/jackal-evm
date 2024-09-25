use cosmwasm_schema::{cw_serde};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {

    PostFile {
        merkle: String,
        file_size: i64,
        proof_interval: i64,
        proof_type: i64,
        max_proofs: i64,
        expires: i64,
        note: String,
    },
    
}

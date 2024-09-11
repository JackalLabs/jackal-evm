use cosmwasm_schema::cw_serde;

// We're making this copy so the linker doesn't complain

#[cw_serde]
pub enum SharedExecuteMsg {

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

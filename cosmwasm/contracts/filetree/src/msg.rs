use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {

    PostKey {
        key: String,
    },

    PostFile {
        merkle: Vec<u8>,
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

/*
// NOTE: Creator field is automatically the contract address
type PostFile struct {
	Merkle        []byte `json:"merkle"`
	FileSize      int64  `json:"file_size"`
	ProofInterval int64  `json:"proof_interval"`
	ProofType     int64  `json:"proof_type"`
	MaxProofs     int64  `json:"max_proofs"`
	Expires       int64  `json:"expires"`
	Note          string `json:"note"`
}

*/
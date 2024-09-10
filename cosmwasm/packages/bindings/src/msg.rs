use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

// A number of Custom messages that can call into the Jackal bindings
#[cw_serde]
pub enum JackalMsg {

    PostKey {
        sender: String, // WARNING: This can be spoofed atm. 
        key: String,
    },
    MakeRoot {
        editors: String,
        viewers: String,
        trackingnumber: String,
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
}

impl JackalMsg {

    pub fn post_key(sender: String, key: String) -> Self {
        JackalMsg::PostKey {
            sender,
            key,
        }
    }

    pub fn post_file(
        merkle: String,
        file_size: i64,
        proof_interval: i64,
        proof_type: i64,
        max_proofs: i64,
        expires: i64,
        note: String,
    ) -> Self {
        JackalMsg::PostFile {
            merkle,
            file_size,
            proof_interval,
            proof_type,
            max_proofs,
            expires,
            note,
        }
    }

    // Not putting sender in just yet 
    pub fn make_root(editors: String, viewers: String, trackingnumber: String) -> Self {
        JackalMsg::MakeRoot {
            editors,
            viewers,
            trackingnumber,
        }
    }
}

impl From<JackalMsg> for CosmosMsg<JackalMsg> {
    fn from(msg: JackalMsg) -> CosmosMsg<JackalMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for JackalMsg {}

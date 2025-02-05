use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

// A number of Custom messages that can call into the Jackal bindings
#[cw_serde]
pub enum JackalMsg {

    // TODO: move post key and make root to where the filetree section should start 
    PostKey {
        // need creator?
        sender: String, // WARNING: This can be spoofed atm. 
        key: String,
    },
    MakeRoot {
        // need creator?
        editors: String,
        viewers: String,
        trackingnumber: String,
    },
    // STORAGE MODULE 
    PostFile {
        creator: String,
        merkle: Vec<u8>,
        file_size: i64,
        proof_interval: i64,
        proof_type: i64,
        max_proofs: i64,
        expires: i64,
        note: String,
    },
    DeleteFile {
        creator: String,
        merkle: Vec<u8>,
        start: i64,
    },
    BuyStorage {
        creator: String,
        for_address: String,
        duration_days: i64,
        bytes: i64,
        payment_denom: String,
        referral: String,
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
        creator: String,
        merkle: Vec<u8>,
        file_size: i64,
        proof_interval: i64,
        proof_type: i64,
        max_proofs: i64,
        expires: i64,
        note: String,
    ) -> Self {
        JackalMsg::PostFile {
            creator,
            merkle,
            file_size,
            proof_interval,
            proof_type,
            max_proofs,
            expires,
            note,
        }
    }

    pub fn delete_file(
        creator: String,
        merkle: Vec<u8>,
        start: i64,
    ) -> Self {
        JackalMsg::DeleteFile {
            creator,
            merkle,
            start,
        }
    }

    pub fn buy_storage(
        creator: String,
        for_address: String,
        duration_days: i64,
        bytes: i64,
        payment_denom: String,
        referral: String,
    ) -> Self {
        JackalMsg::BuyStorage {
            creator,
            for_address,
            duration_days,
            bytes,
            payment_denom,
            referral,
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

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

}

impl JackalMsg {

    pub fn post_key(sender: String, key: String) -> Self {
        JackalMsg::PostKey {
            sender,
            key,
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

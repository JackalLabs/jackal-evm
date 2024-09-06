use cosmwasm_schema::cw_serde;

// We're making this copy so the linker doesn't complain

#[cw_serde]
pub enum SharedExecuteMsg {

    PostKey {
        key: String,
    },

    MakeRoot {
        editors: String,
        viewers: String,
        trackingnumber: String,
    },
    
}

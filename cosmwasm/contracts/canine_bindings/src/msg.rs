use cosmwasm_schema::{cw_serde, QueryResponses};

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

    DeleteFile {
        merkle: String,
        start: i64,
    },
    
    BuyStorage {
        for_address: String,
        duration_days: i64,
        bytes: i64,
        payment_denom: String,
        referral: String,
    },

    RequestReportForm {
        prover: String,
        merkle: String,
        owner: String,
        start: i64,
    },

    PostFileTree {
        account: String,
        hash_parent: String,
        hash_child: String,
        contents: String,
        viewers: String,
        editors: String,
        tracking_number: String,
    },

    AddViewers {
        viewer_ids: String,
        viewer_keys: String,
        address: String,
        file_owner: String,
    },

    PostKey {
        key: String,
    },

    DeleteFileTree {
        hash_path: String, 
        account: String,
    },

    RemoveViewers {
        viewer_ids: String, 
        address: String, 
        file_owner: String,
    }, 

    ProvisionFileTree {
        editors: String, 
        viewers: String, 
        tracking_number: String,
    },

    AddEditors {
        editor_ids: String, 
        editor_keys: String, 
        address: String, 
        file_owner: String,
    },

    RemoveEditors {
        editor_ids: String, 
        address: String, 
        file_owner: String,
    },

    ResetEditors {
        address: String, 
        file_owner: String,
    },

    ResetViewers {
        address: String, 
        file_owner: String,
    },

    ChangeOwner {
        address: String, 
        file_owner: String,
        new_owner: String,
    },
    
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// GetContractState returns the contact's state.
    #[returns(crate::state::ContractState)]
    GetContractState {},
}

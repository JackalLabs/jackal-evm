use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

// A number of Custom messages that can call into the Jackal bindings
#[cw_serde]
pub enum JackalMsg {

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
    RequestReportForm {
        creator: String,
        prover: String,
        merkle: Vec<u8>,
        owner: String,
        start: i64,
    },
    // FILETREE MODULE
    PostFileTree {
        creator: String,
        account: String,
        hash_parent: String,
        hash_child: String,
        contents: String,
        viewers: String,
        editors: String,
        tracking_number: String,
    },
    AddViewers {
        creator: String,
        viewer_ids: String, 
        viewer_keys: String,
        address: String, 
        file_owner: String,
    },
    PostKey {
        creator: String,
        key: String,
    },
    DeleteFileTree {
        creator: String,
        hash_path: String, 
        account: String,

    },
    RemoveViewers {
        creator: String,
        viewer_ids: String, 
        address: String, 
        file_owner: String,
    },
    ProvisionFileTree {
        creator: String,
        editors: String, 
        viewers: String, 
        tracking_number: String,
    },
    AddEditors {
        creator: String,
        editor_ids: String, 
        editor_keys: String, 
        address: String, 
        file_owner: String,
    },
    RemoveEditors {
        creator: String,
        editor_ids: String, 
        address: String, 
        file_owner: String,
    },
    ResetEditors {
        creator: String,
        address: String, 
        file_owner: String,
    },
}

impl JackalMsg {

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

    pub fn request_report_form(
        creator: String,
        prover: String,
        merkle: Vec<u8>,
        owner: String,
        start: i64,
    ) -> Self {
        JackalMsg::RequestReportForm {
            creator,
            prover,
            merkle,
            owner,
            start,
        }
    }

    pub fn post_file_tree(
        creator: String,
        account: String,
        hash_parent: String,
        hash_child: String,
        contents: String,
        viewers: String,
        editors: String,
        tracking_number: String,
    ) -> Self {
        JackalMsg::PostFileTree {
            creator,
            account,
            hash_parent,
            hash_child,
            contents,
            viewers,
            editors,
            tracking_number,
        }
    }

    pub fn add_viewers(
        creator: String, 
        viewer_ids: String,
        viewer_keys: String, 
        address: String,
        file_owner: String, 
    ) -> Self {
        JackalMsg::AddViewers { 
            creator, 
            viewer_ids, 
            viewer_keys, 
            address, 
            file_owner,
        } 
    }

    pub fn post_key(
        creator: String, 
        key: String
    ) -> Self {
        JackalMsg::PostKey {
            creator,
            key,
        }
    }

    pub fn delete_file_tree(
        creator: String,
        hash_path: String, 
        account: String
    ) -> Self {
        JackalMsg::DeleteFileTree {
            creator,
            hash_path,
            account
        }
    }

    pub fn remove_viewers(
        creator: String,
        viewer_ids: String, 
        address: String, 
        file_owner: String,
    ) -> Self {
        JackalMsg::RemoveViewers {
            creator,
            viewer_ids,
            address,
            file_owner
        }
    }

    pub fn provision_file_tree(
        creator: String,
        editors: String, 
        viewers: String, 
        tracking_number: String,
    ) -> Self {
        JackalMsg::ProvisionFileTree {
            creator,
            editors, 
            viewers, 
            tracking_number
        }
    }

    pub fn add_editors(
        creator: String,
        editor_ids: String, 
        editor_keys: String, 
        address: String, 
        file_owner: String,
    ) -> Self {
        JackalMsg::AddEditors {
            creator,
            editor_ids, 
            editor_keys, 
            address, 
            file_owner
        }
    }

    pub fn remove_editors(
        creator: String,
        editor_ids: String, 
        address: String, 
        file_owner: String,
    ) -> Self {
        JackalMsg::RemoveEditors {
            creator,
            editor_ids, 
            address, 
            file_owner
        }
    }

    pub fn reset_editors(
        creator: String,
        address: String, 
        file_owner: String,
    ) -> Self {
        JackalMsg::ResetEditors {
            creator,
            address, 
            file_owner
        }
    }
}

impl From<JackalMsg> for CosmosMsg<JackalMsg> {
    fn from(msg: JackalMsg) -> CosmosMsg<JackalMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for JackalMsg {}

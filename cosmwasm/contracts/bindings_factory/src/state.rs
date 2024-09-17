use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub use contract::ContractState;

/// The item used for storing the bindings contract's code id 
/// TODO: need a function to update the code ID when we release an updated version of the outpost 
pub const STATE: Item<ContractState> = Item::new("state");

/// A mapping of the user's evm address to the bindings contract address they own 
/// WARNING: NOTE - the value here used to be 'String'
pub const USER_ADDR_TO_BINDINGS_ADDR: Map<&str, String> = Map::new("user_addr_to_bindings_addr");

/// TODO: add owner white list 

mod contract {

    use super::*;

    #[cw_serde]
    pub struct ContractState {
        /// The code ID of the bindings contract.
        pub bindings_code_id: u64,
        pub owner: String,
    }

    impl ContractState {
        /// Creates a new ContractState.
        pub fn new(bindings_code_id: u64, owner: String) -> Self {
            Self {
                bindings_code_id,
                owner,
            }
        }
    }
}



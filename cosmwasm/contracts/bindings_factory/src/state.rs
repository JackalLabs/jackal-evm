use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub use contract::ContractState;

/// The item used for storing the bindings contract's code id 
/// TODO: need a function to update the code ID when we release an updated version of the outpost 
pub const STATE: Item<ContractState> = Item::new("state");

/// A mapping of the user's evm address to the bindings contract address they own 
pub const USER_ADDR_TO_BINDINGS_ADDR: Map<&str, String> = Map::new("user_addr_to_bindings_addr");

/// This behaves like a lock file which ensures that users can only create bindings for themselves
/// It's a needed work around that's caused by inter-contract executions being signed by the calling contract instead of the user's signature
pub const LOCK: Map<&str, bool> = Map::new("lock");

mod contract {

    use super::*;

    #[cw_serde]
    pub struct ContractState {
        /// The code ID of the bindings contract.
        pub bindings_code_id: u64,
    }

    impl ContractState {
        /// Creates a new ContractState.
        pub fn new(bindings_code_id: u64) -> Self {
            Self {
                bindings_code_id,
            }
        }
    }
}




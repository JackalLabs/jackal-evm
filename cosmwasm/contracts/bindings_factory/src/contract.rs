#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractState, STATE};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bindings-factory"; // just a placeholder, not yet published
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // NOTE: admin should be set in the wasm.Instanstiate protobuf msg
    // Setting it into contract state is actually useless when wasmd checks for migration permissions
    
    // TODO: set owner?

    STATE.save(
        deps.storage,
        &ContractState::new(msg.bindings_code_id),
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateBindings {} => execute::create_bindings(deps, env, info),
        ExecuteMsg::CreateBindingsV2 {
            user_evm_address
        } => execute::create_bindings_v2(deps, env, info, user_evm_address),
        ExecuteMsg::MapUserBindings {} => execute::map_user_bindings(deps, env, info),
        ExecuteMsg::CallBindings { evm_address, msg } => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractState {} => to_json_binary(&query::state(deps)?),
        QueryMsg::GetUserBindingsAddress { user_address } => to_json_binary(&query::user_bindings_address(deps, user_address)?),
    }
}

mod execute {
    use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Uint128, Event, to_json_binary};
    use crate::state::{self, USER_ADDR_TO_BINDINGS_ADDR, LOCK};

    use filetree::{bindings_helpers::BindingsCode, msg::InstantiateMsg};

    use super::*;

    pub fn create_bindings(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;
        // WARNING: This function is called by the user, so we cannot error:unauthorized if info.sender != admin 

        let bindings_code_id = BindingsCode::new(state.bindings_code_id);

        // Check if key already exists and disallow multiple bindings creations 
        // If key exists, we don't care what the address is, just the mere existence of the key means an bindings was 
        // already created
            
        if let Some(value) = USER_ADDR_TO_BINDINGS_ADDR.may_load(deps.storage, &info.sender.to_string())? {
            return Err(ContractError::AlreadyCreated(value))
        }

        // If we set the lock to be the owner of the factory -- do we even really need the lock?
        let _lock = LOCK.save(deps.storage, &info.sender.to_string(), &true);

        // TODO: use the callback again
        // let callback = Callback {};

        let instantiate_msg = filetree::msg::InstantiateMsg {};

        let label
         = format!("bindings contract-owned by: {}", &info.sender.to_string());

        // 'instantiate2' has the ability to pre compute the binding's contract address
        // We are only instantiating on Jackal--if 'instantiate2' works on Jackal, we can get rid of the lock and callback mechanism 
        // And we can save it right away

        let cosmos_msg = bindings_code_id.instantiate(
            instantiate_msg,
            label,
            Some(info.sender.to_string()),
        )?;

        let mut event = Event::new("FACTORY: create_binding");
        event = event.add_attribute("info.sender", &info.sender.to_string());

        Ok(Response::new().add_message(cosmos_msg).add_event(event)) 
    }

    pub fn create_bindings_v2(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        user_evm_address: String,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;
        // WARNING: This function is called by the user, so we cannot error:unauthorized if info.sender != admin 

        let bindings_code_id = BindingsCode::new(state.bindings_code_id);

        // Check if key already exists and disallow multiple bindings creations 
        // If key exists, we don't care what the address is, just the mere existence of the key means an bindings was 
        // already created
        
        // If bindings contract already made for this account, don't make another one
        if let Some(value) = USER_ADDR_TO_BINDINGS_ADDR.may_load(deps.storage, &user_evm_address)? {
            return Err(ContractError::AlreadyCreated(value))
        }

        // TODO: Because instantiate2 works, I don't think we even need this lock now
        // Set this such that only the owner of the factory can call it 

        // If we set the lock to be the owner of the factory -- do we even really need the lock?
        let _lock = LOCK.save(deps.storage, &info.sender.to_string(), &true);

        // TODO: use the callback again
        // let callback = Callback {};

        let instantiate_msg = filetree::msg::InstantiateMsg {};

        let label
         = format!("bindings contract-owned by: {}", &user_evm_address);

        // 'instantiate2' has the ability to pre compute the binding's contract address
        // We are only instantiating on Jackal--if 'instantiate2' works on Jackal, we can get rid of the lock and callback mechanism 
        // And we can save it right away

        let (cosmos_msg, contract_addr) = bindings_code_id.instantiate2(
            deps.api,
            &deps.querier,
            &env,
            instantiate_msg,
            label,
            Some(env.contract.address.to_string()),
            // WARNING: is it okay to use current block time as salt? The ica-controller only uses this as a fallback option
            env.block.time.seconds().to_string(), 
        )?;

        // TODO: map evm address <> bindings contract here 

        let mut event = Event::new("FACTORY: create_binding");
        event = event.add_attribute("pre-computed bindings contract address:", contract_addr.as_str()); // WARNING: not 100% sure 'as_str' returns bech32 format

        Ok(Response::new().add_message(cosmos_msg).add_event(event)) 
    }

    pub fn map_user_bindings(
        deps: DepsMut,
        env: Env,
        info: MessageInfo, //info.sender will be the bindings's address 
        // bindings_owner: String, 
    ) -> Result<Response, ContractError> {
        // Mapping needed:
        // evm address <> bindings address
        // because of cross contract calls, the info.sender of this would be the bindings address, so we would need the evm address
        // to be propagated from above

        // TODO: 
        // if instantiate2 worked, we don't need this function

        // this contract can't have an owner because it needs to be called back by every bindings it instantiates 

        // Load the lock state for the bindings owner
        let lock = LOCK.may_load(deps.storage, &"evm address goes here")?; // WARNING-just hardcoding for testing 

        // Check if the lock exists and is true
        if let Some(true) = lock {
            // If it does, overwrite it with false
            LOCK.save(deps.storage, &"evm address goes here", &false)?;
        } else {
            // This function can only get called if the Lock was set in 'create_bindings'
            // If it doesn't exist or is false, return an unauthorized error

            // TODO: put error back
            // return Err(ContractError::MissingLock {  })
        }

    USER_ADDR_TO_BINDINGS_ADDR.save(deps.storage, &"evm address goes here", &info.sender.to_string())?; // again, info.sender is actually the bindings address

    let mut event = Event::new("FACTORY:map_bindings_bindings");
        event = event.add_attribute("info.sender", &info.sender.to_string());

    // DOCUMENT: note in README that a successful bindings creation shall return the address in the tx.res.attribute 
    // and a failure will throw 'AlreadyCreated' contractError

    // NOTE: calling '.add_attribute' just adds a key value pair to the main wasm attribute 
    // WARNING: is it possible at all that these bytes are non-deterministic?
    // This can't be because we take from 'info.sender' which only exists if this function is called in the first place
    // This function is called only if the bindings executes the callback, otherwise the Tx was abandoned while sitting in the 
    // mem pool

    Ok(Response::new().add_event(event)) // this data is not propagated back up to the tx resp of the 'create_bindings' call
    }
}

mod query {
    use crate::state::USER_ADDR_TO_BINDINGS_ADDR;

    use super::*;

    /// Returns the saved contract state.
    pub fn state(deps: Deps) -> StdResult<ContractState> {
        STATE.load(deps.storage)
    }

    /// Returns the bindings address this user owns
    pub fn user_bindings_address(deps: Deps, user_address: String) -> StdResult<String> {
        USER_ADDR_TO_BINDINGS_ADDR.load(deps.storage, &user_address)
    }
}

#[cfg(test)]
mod tests {}





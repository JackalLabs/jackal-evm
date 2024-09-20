#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractState, STATE, WHITE_LIST};

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
    
    STATE.save(
        deps.storage,
        &ContractState::new(msg.bindings_code_id, info.sender.to_string()),
    )?;
    // Add the owner to the white list of senders
    WHITE_LIST.save(deps.storage, &info.sender.to_string(), &true)?; // again, info.sender is actually the outpost address

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
        ExecuteMsg::CallBindings { evm_address, msg } => execute::call_bindings(deps, env, info, evm_address, msg),
        ExecuteMsg::AddToWhiteList { jkl_address } => execute::add_to_white_list(deps, env, info, jkl_address),

    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractState {} => to_json_binary(&query::state(deps)?),
        QueryMsg::GetUserBindingsAddress { user_address } => to_json_binary(&query::user_bindings_address(deps, user_address)?),
        QueryMsg::GetAllUserBindingsAddresses {} => to_json_binary(&query::all_user_bindings_addresses(deps)?),
    }
}

mod execute {
    use cosmwasm_std::{CosmosMsg, Event, WasmMsg};
    use crate::state::{USER_ADDR_TO_BINDINGS_ADDR, WHITE_LIST};
    use shared::shared_msg::SharedExecuteMsg;

    use filetree::bindings_helpers::{BindingsCode, BindingsContract};

    use super::*;

    pub fn call_bindings(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        evm_address: String,
        msg: SharedExecuteMsg
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;

        let mut allowed: bool = false;

        // Use may_load to attempt to retrieve the value associated with the key
        if let Some(value) = WHITE_LIST.may_load(deps.storage, &info.sender.to_string())? {
            // If the key exists, return the value
            allowed = value
        } 

        if allowed == false {
            return Err(ContractError::NotAllowed())
        }

        let mut bindings_address: String = String::new();

        // declare empty cosmos msg here to be assigned by else block:
        let mut factory_cosmos_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate2 {
            admin: None, // WARNING: TODO: the owner of the factory should be admin so it can perform migrations on bindings
            code_id: 0,
            label: String::new(),
            msg: Binary::default(),
            funds: vec![],
            salt: Binary::default(),
        });

        // Use may_load to attempt to retrieve the value associated with the key
        if let Some(value) = USER_ADDR_TO_BINDINGS_ADDR.may_load(deps.storage, &evm_address)? {
        // If the key exists, return the value
        bindings_address = value
        } else {
        // If the key does not exist, return the custom error
            let bindings_code_id = BindingsCode::new(state.bindings_code_id);
            let instantiate_msg = filetree::msg::InstantiateMsg {};

            let label
            = format!("bindings contract-owned by: {}", &evm_address);

            let (instantiate2_cosmos_msg, bindings_contract_address) = bindings_code_id.instantiate2(
                deps.api,
                &deps.querier,
                &env,
                instantiate_msg,
                label,
                Some(env.contract.address.to_string()),
                // WARNING: is it okay to use current block time as salt? The ica-controller only uses this as a fallback option
                env.block.time.seconds().to_string(), 
            )?;

            factory_cosmos_msg = instantiate2_cosmos_msg;

            USER_ADDR_TO_BINDINGS_ADDR.save(deps.storage, &evm_address, &bindings_contract_address.to_string())?; // again, info.sender is actually the outpost address
            let mut event = Event::new("FACTORY: create_binding");
            bindings_address = bindings_contract_address.to_string();

        }

        // Convert the bech32 string back to 'Addr' type before passing to the filetree helper API
        let error_msg: String = String::from("Bindings contract address is not a valid bech32 address. Conversion back to addr failed");
        let bindings_contract = BindingsContract::new(deps.api.addr_validate(&bindings_address).expect(&error_msg));
        
        // Execute the bindings contract with given msg
        let cosmos_msg = bindings_contract.execute(msg, info.funds)?;

        // Make sure factory_cosmos_msg is not empty

        let mut messages: Vec<CosmosMsg> = Vec::new();

        let mut id: u64 = 0;

        if let CosmosMsg::Wasm(wasm_msg) = factory_cosmos_msg.clone() {
           if let WasmMsg::Instantiate2 { admin: _, code_id, label: _, msg: _, funds: _, salt: _ } = wasm_msg {
                id = code_id;
           }
        }

        if id != 0 {
            messages.push(factory_cosmos_msg);
        }
        messages.push(cosmos_msg);
        
        Ok(Response::new().add_messages(messages)) 
    }

    pub fn add_to_white_list(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        jkl_address: String,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;

        // Only the factory owner can add an address to the white list
        if info.sender.to_string() != state.owner {
            return Err(ContractError::CannotUpdate())
        }

        WHITE_LIST.save(deps.storage, &jkl_address, &true)?; // again, info.sender is actually the outpost address

        Ok(Response::new()) 

    }

}

mod query {
    use cosmwasm_std::Order;

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

    /// Returns the entire map of user addresses to their bindings addresses
    pub fn all_user_bindings_addresses(deps: Deps) -> StdResult<Vec<(String, String)>> {
        let mut all_bindings = vec![];

        let iter = USER_ADDR_TO_BINDINGS_ADDR.range(deps.storage, None, None, Order::Ascending);
        for item in iter {
            let (key, value) = item?;
            all_bindings.push((key.to_string(), value));
        }

        Ok(all_bindings)
    }

    
}

#[cfg(test)]
mod tests {}
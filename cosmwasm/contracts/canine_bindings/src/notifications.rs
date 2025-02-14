#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Binary
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractState, STATE};
use jackal_bindings::JackalMsg;

pub fn create_notification(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    to: String,
    contents: String,
    private_contents: String,
) -> Result<Response<JackalMsg>, ContractError> {

    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner.to_string() {
        return Err(ContractError::Unauthorized {})
    }

    let private_contents_bytes = cosmwasm_std::Binary::from_base64(&private_contents).expect("could not get private contents from base64");

    let creator = env.contract.address.to_string();

    let create_notification_msg = JackalMsg::create_notification(
        creator,
        to,
        contents,
        private_contents_bytes.to_vec(),
    );

    let res = Response::new()
        .add_attribute("method", "create_notification")
        .add_message(create_notification_msg);
    Ok(res)
}

pub fn delete_notification(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    from: String,
    time: i64,
) -> Result<Response<JackalMsg>, ContractError> {

    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner.to_string() {
        return Err(ContractError::Unauthorized {})
    }

    let creator = env.contract.address.to_string();

    let delete_notification_msg = JackalMsg::delete_notification(
        creator,
        from,
        time,
    );

    let res = Response::new()
        .add_attribute("method", "delete_notification")
        .add_message(delete_notification_msg);
    Ok(res)
}

pub fn block_senders(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    to_block: Vec<String>
) -> Result<Response<JackalMsg>, ContractError> {

    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner.to_string() {
        return Err(ContractError::Unauthorized {})
    }

    let creator = env.contract.address.to_string();

    let block_senders_msg = JackalMsg::block_senders(
        creator,
        to_block,
    );

    let res = Response::new()
        .add_attribute("method", "block_senders")
        .add_message(block_senders_msg);
    Ok(res)
}
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Binary
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractState, STATE};
use jackal_bindings::JackalMsg;

pub fn post_file_tree(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    account: String,
    hash_parent: String,
    hash_child: String,
    contents: String,
    viewers: String,
    editors: String,
    tracking_number: String,
) -> Result<Response<JackalMsg>, ContractError> {

    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner.to_string() {
        return Err(ContractError::Unauthorized {})
    }

    let creator = env.contract.address.to_string();

    let post_file_tree_msg = JackalMsg::post_file_tree(
        creator,
        account,
        hash_parent,
        hash_child,
        contents,
        viewers,
        editors,
        tracking_number,
    );

    let res = Response::new()
        .add_attribute("method", "post_file_tree")
        .add_message(post_file_tree_msg);
    Ok(res)
}

pub fn add_viewers(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    viewer_ids: String,
    viewer_keys: String,
    address: String,
    file_owner: String,
) -> Result<Response<JackalMsg>, ContractError> {

    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner.to_string() {
        return Err(ContractError::Unauthorized {})
    }

    let creator = env.contract.address.to_string();

    let add_viewers = JackalMsg::add_viewers(
        creator,
        viewer_ids,
        viewer_keys,
        address,
        file_owner
    );

    let res = Response::new()
        .add_attribute("method", "add_viewers")
        .add_message(add_viewers);
    Ok(res)
}

pub fn post_key(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    key: String,
) -> Result<Response<JackalMsg>, ContractError> {

    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner.to_string() {
        return Err(ContractError::Unauthorized {})
    }

    let creator = env.contract.address.to_string();

    let post_key = JackalMsg::post_key(
        creator,
        key,
    );

    let res = Response::new()
        .add_attribute("method", "post_key")
        .add_message(post_key);
    Ok(res)
}
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{State, STATE};
use jackal_bindings::JackalMsg;

// Consider adding migration info?

#[cfg(not(feature = "no_exports"))] 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg(not(feature = "no_exports"))] 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<JackalMsg>, ContractError> {
    match msg {
        ExecuteMsg::PostFile {
            merkle, 
            file_size, 
            proof_interval, 
            proof_type, 
            max_proofs, 
            expires, 
            note } => post_file(
                deps,
                info, 
                env,
                merkle, 
                file_size, 
                proof_interval, 
                proof_type, 
                max_proofs, 
                expires, 
                note
            ),
    }
}

pub fn post_file(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    merkle: String,
    file_size: i64,
    proof_interval: i64,
    proof_type: i64,
    max_proofs: i64,
    expires: i64,
    note: String,
) -> Result<Response<JackalMsg>, ContractError> {

    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner.to_string() {
        return Err(ContractError::Unauthorized {})
    }

    // WARNING: TODO: Does canine-bindings itself need to ensure only white listed addresses can sign?

    let merkle_bytes = cosmwasm_std::Binary::from_base64(&merkle).expect("could not get merkle from base64");

    let creator = env.contract.address.to_string();
    // Checks and validations go here?
    let post_file_msg = JackalMsg::post_file(
        creator,
        merkle_bytes.to_vec(),
        file_size,
        proof_interval,
        proof_type,
        max_proofs,
        expires,
        note,
    );

    let res = Response::new()
        .add_attribute("method", "post_file")
        .add_message(post_file_msg);
    Ok(res)
}



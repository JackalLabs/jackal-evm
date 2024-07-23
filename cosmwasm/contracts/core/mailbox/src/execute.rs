use cosmwasm_std::{
    ensure, ensure_eq, to_json_binary, wasm_execute, Coin, Coins, DepsMut, Env, HexBinary,
    MessageInfo, Response,
};

use hpl_interface::{
    core::{
        mailbox::{DispatchMsg, DispatchResponse},
        HandleMsg,
    },
    hook::{post_dispatch, quote_dispatch},
    ism,
    types::Message,
};

use crate::{
    event::{
        emit_default_ism_set
    },
    state::{Delivery, CONFIG, DELIVERIES, LATEST_DISPATCHED_ID, NONCE},
    ContractError, MAILBOX_VERSION,
};

pub fn set_default_ism(
    deps: DepsMut,
    info: MessageInfo,
    new_default_ism: String,
) -> Result<Response, ContractError> {
    
    // TODO: check ownership with cw-ownable because we can't find hpl_ownable crate

    let new_default_ism = deps.api.addr_validate(&new_default_ism)?;
    let event = emit_default_ism_set(info.sender, new_default_ism.clone());

    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        config.default_ism = Some(new_default_ism);

        Ok(config)
    })?;

    Ok(Response::new().add_event(event))
}
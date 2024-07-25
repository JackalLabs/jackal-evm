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
        emit_default_ism_set, emit_process,
        emit_process_id,
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

pub fn process(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;

    let decoded_msg: Message = message.into();
    let recipient = decoded_msg.recipient_addr(&config.hrp)?; // hrp stands for 'hyperlane recipient prefix'? 

    // solidity mailbox version must match cosmwasm mailbox version
    // solidity mailbox recipient domain must match cosmwasm mailbox's local domain 
    ensure_eq!(
        decoded_msg.version,
        MAILBOX_VERSION,
        ContractError::InvalidMessageVersion {
            version: decoded_msg.version
        }
    );
    ensure_eq!(
        decoded_msg.dest_domain,
        config.local_domain,
        ContractError::InvalidDestinationDomain {
            domain: decoded_msg.dest_domain
        }
    );

    let id = decoded_msg.id();
    let ism = ism::recipient(&deps.querier, &recipient)?.unwrap_or(config.get_default_ism()); // hopefully the default will return

    // every message sent from EVM will have a unique id. We shall keep track of these ids and make sure the same message isn't sent twice
    // was likely a feature to help prevent re-entrancy
    ensure!(
        !DELIVERIES.has(deps.storage, id.to_vec()),
        ContractError::AlreadyDeliveredMessage {}
    );

    DELIVERIES.save(
        deps.storage,
        id.to_vec(),
        &Delivery {
            sender: info.sender,
            block_number: env.block.height,
        },
    )?;

    // For now we will use a no-op ism that shall return true without doing any checks
    let verify = ism::verify(&deps.querier, ism, metadata, decoded_msg.clone().into())?;

    deps.api
    .debug(&format!("mailbox::process: verify: {}", verify));

    ensure!(verify, ContractError::VerifyFailed {});

    // This all makes sense, but how does the relayer even cause this process function to execute in the first place?
    // How will it find the execute entry point?
    let handle_msg = wasm_execute(
        recipient,
        &HandleMsg {
            origin: decoded_msg.origin_domain,
            sender: decoded_msg.sender.clone(),
            body: decoded_msg.body,
        }
        .wrap(),
        vec![],
    )?;


    Ok(Response::new().add_message(handle_msg).add_events(vec![
        emit_process_id(id),
        emit_process(
            decoded_msg.origin_domain, 
            // supposed to emit the origin domain the msg got sent from? 
            // before, this was 'config.local_domain'
            decoded_msg.sender,
            decoded_msg.recipient,
        ),
    ]))
}
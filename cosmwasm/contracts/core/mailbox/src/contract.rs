#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ensure, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse, Response};

use hpl_interface::{
    core::mailbox::{InstantiateMsg, MailboxHookQueryMsg, MailboxQueryMsg, QueryMsg},
    to_binary,
};

use hpl_interface::core::mailbox::ExecuteMsg as ExternalExecuteMsg;

use crate::{
    error::ContractError,
    event::emit_instantiated,
    state::{Config, CONFIG, NONCE},
    CONTRACT_NAME, CONTRACT_VERSION,
    msg::ExecuteMsg,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // check hrp is lowercase
    ensure!(
        msg.hrp.chars().all(|v| v.is_lowercase()),
        ContractError::invalid_config("hrp must be lowercase")
    );

    let config = Config {
        hrp: msg.hrp,
        local_domain: msg.domain,
        default_ism: None, // NOTE: these can be un set on init?
        default_hook: None,
        required_hook: None,
    };

    let owner = deps.api.addr_validate(&msg.owner)?;

    CONFIG.save(deps.storage, &config)?;
    NONCE.save(deps.storage, &0u32)?;

    // WARNING: we could not find the below package. Perhaps 'many things' team did not publish
    // Perhaps we can use cw_ownable library instead?

    // hpl_ownable::initialize(deps.storage, &owner)?;

    Ok(Response::new().add_event(emit_instantiated(owner)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;
    use ExecuteMsg::*;

    // If Jackal is only using the mailbox as the receiver from EVM, we don't need ownable, hooks, or dispatch, atm? 
    match msg {
        ExecuteMsg::External(external_msg) => {
            // Handle the external enum variants imported from hpl-interface
            match external_msg {
                ExternalExecuteMsg::Ownable(_) => todo!(),
                ExternalExecuteMsg::SetDefaultIsm { ism } => execute::set_default_ism(deps, info, ism),
                ExternalExecuteMsg::SetDefaultHook { hook } => todo!(),
                ExternalExecuteMsg::SetRequiredHook { hook } => todo!(),
                ExternalExecuteMsg::Dispatch(_) => todo!(),
                ExternalExecuteMsg::Process { metadata, message } => execute::process(deps, env, info, metadata, message),
            }
        },
        ExecuteMsg::Signer { } => {
            execute::signer(deps, env, info)
        }
    }

}

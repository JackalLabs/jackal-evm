#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ensure, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse, Response};

use hpl_interface::{
    core::mailbox::{ExecuteMsg, InstantiateMsg, MailboxHookQueryMsg, MailboxQueryMsg, QueryMsg},
    to_binary,
};

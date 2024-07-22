use cosmwasm_std::{Addr, Event, HexBinary};
use hpl_interface::types::Message;

pub fn emit_instantiated(owner: Addr) -> Event {
    Event::new("mailbox_instantiated").add_attribute("owner", owner)
}
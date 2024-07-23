use cosmwasm_std::{Addr, Event, HexBinary};
use hpl_interface::types::Message;

pub fn emit_instantiated(owner: Addr) -> Event {
    Event::new("mailbox_instantiated").add_attribute("owner", owner)
}

pub fn emit_default_ism_set(owner: Addr, new_default_ism: Addr) -> Event {
    Event::new("mailbox_default_ism_set")
        .add_attribute("owner", owner)
        .add_attribute("new_default_ism", new_default_ism)
}
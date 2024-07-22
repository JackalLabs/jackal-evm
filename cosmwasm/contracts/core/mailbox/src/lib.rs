pub mod contract;
mod state;

pub const MAILBOX_VERSION: u8 = 3;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

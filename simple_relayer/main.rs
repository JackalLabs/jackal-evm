//! The relayer forwards signed checkpoints from the current chain's mailbox to 
//! the other chains' mailboxes
//! 
//! At a regular interval, the relayer polls the current chain's mailbox for 
//! signed checkpoints and submits them as checkpoints on the remote mailbox.
//! 
//! 
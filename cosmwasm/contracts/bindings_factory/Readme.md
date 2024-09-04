# bindings factory 

This contract will mint bindings contracts for each user, and map the user's evm address to their bindings contract address on canine-chain.

The goal:

The factory will be responsible for executing each user's bindings contract to make filetree and storage entries. Those entries will
be filled with the following values:

{
  sender: factory_contract_address
  creator: user_bindings_contract_address
  ...other values...
}

The assignment of the 'sender' field is controlled by the CosmWasm framework's cross contract calls.

The assignment of the 'creator' field is controlled by our 'wasmbindings' package in canine-chain.

Users are not permitted to overwrite or set these fields. 


# cosmwasm-mailbox

Everything needed to get the core mailbox deployed on local canine-chain (jackal-chain) and integrated with our filetree and storage modules.

It is likely that the mailbox contract will call the wasmbindings contract on jackal.

## Building the contracts

Run the following command in the root directory of this repository:

### `mailbox`

```text
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="devcontract_cache_burner",target=/code/contracts/burner/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.1 /code/cosmwasm/contracts/core/mailbox

```
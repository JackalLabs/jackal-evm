# filetree wasm bindings 

Web assembly bindings contract to call into canine-chain's filetree module. We might integrate this code directly into the mailbox. 

## Building the contract

Run the following command in the root directory of this repository:

### `filetree`

```text
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="devcontract_cache_burner",target=/code/contracts/burner/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.1 /code/cosmwasm/contracts/filetree

```
GOAL:
	To pull down this Docker container: docker pull --platform linux/amd64 gcr.io/abacus-labs-dev/hyperlane-agent:7a8478b-20240703-113821

	And run it by filling out the placeholders in this command:
		docker run \
		-it \
		-e CONFIG_FILES=<agent_config.json path> \
		--mount type=bind,source=$CONFIG_FILES,target=<agent_config.json path>,readonly \
		--mount type=bind,source="$(pwd)"/hyperlane_db_relayer,target=/hyperlane_db \
		--mount type=bind,source="$(pwd)"/$VALIDATOR_SIGNATURES_DIR,target=/tmp/validator-signatures,readonly \
		gcr.io/abacus-labs-dev/hyperlane-agent:7a8478b-20240703-113821 \
		./relayer \
		--db /hyperlane_db \
		--relayChains <chain_1_name>,<chain_2_name> \
		--allowLocalCheckpointSyncers true \

1. Make sure you're on Node version 18.16.0

2. hyperlane registry init
	Init a registry entry for your local chain


4. hyperlane core init --advance
	- messageIdMultisigIsm
	- Addr: One generated before
	- Hook type: Guess, but 
	
	
	hyperlane core deploy --dry-run localeth
    --from-address <Test Private Key>


https://docs.hyperlane.xyz/docs/guides/deploy-hyperlane-local-agents#4-run-a-relayer
docker pull --platform linux/amd64 gcr.io/abacus-labs-dev/hyperlane-agent:7a8478b-20240703-113821
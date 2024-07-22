1. cast send <Test Public Key> \
--private-key <Test Private Key> \
--value $(cast tw 1)

2. Make sure you're on Node version 18.16.0

3. hyperlane registry init

4. hyperlane core init --advance
	- messageIdMultisigIsm
	- Addr: One generated before
	- Hook type: Guess, but 
	
	
	hyperlane core deploy --dry-run localeth
    --from-address <Test Private Key>


https://docs.hyperlane.xyz/docs/guides/deploy-hyperlane-local-agents#4-run-a-relayer
docker pull --platform linux/amd64 gcr.io/abacus-labs-dev/hyperlane-agent:7a8478b-20240703-113821
1. cast send 0x1D7519FC24be7ad9948427c866440c997975b9A0 \
--private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
--value $(cast tw 1)

2. Make sure you're on Node version 18.16.0

3. hyperlane registry init

4. hyperlane core init --advance
	- messageIdMultisigIsm
	- Addr: One generated before
	- Hook type: Guess, but 
	
	
	hyperlane core deploy --dry-run localeth
    --from-address 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266


https://docs.hyperlane.xyz/docs/guides/deploy-hyperlane-local-agents#4-run-a-relayer
docker pull --platform linux/amd64 gcr.io/abacus-labs-dev/hyperlane-agent:7a8478b-20240703-113821
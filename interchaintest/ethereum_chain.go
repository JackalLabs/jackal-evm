package interchaintest

import (
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

// Having trouble finding an ARM docker build of foundry so we're just using a local one for now

func LocalEthereumAnvilChainConfig(
	name string,
) ibc.ChainConfig {
	return ibc.ChainConfig{
		Type:           "ethereum",
		Name:           name,
		ChainID:        "31337", // default anvil chain-id
		Bech32Prefix:   "n/a",
		CoinType:       "60",
		Denom:          "wei",
		GasPrices:      "0",
		GasAdjustment:  0,
		TrustingPeriod: "0",
		NoHostMount:    false,
		Images: []ibc.DockerImage{
			{
				Repository: "foundry",
				Version:    "latest",
				// UidGid:     "1000:1000",
			},
		},
		Bin: "anvil",
	}
}

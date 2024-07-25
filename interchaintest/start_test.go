package interchaintest

import (
	"context"
	"fmt"
	"testing"
	"time"

	"cosmossdk.io/math"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap/zaptest"

	"github.com/strangelove-ventures/interchaintest/v7"
	"github.com/strangelove-ventures/interchaintest/v7/chain/ethereum"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
)

// NOTE: Foundry comes with pre loaded accounts, and private keys with seed phrases

func TestEthereum(t *testing.T) {

	if testing.Short() {
		t.Skip()
	}

	t.Parallel()

	client, network := interchaintest.DockerSetup(t)

	// Log location
	f, err := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
	require.NoError(t, err)
	// Reporter/logs
	rep := testreporter.NewReporter(f)
	eRep := rep.RelayerExecReporter(t)

	ctx := context.Background()

	// Get default ethereum chain config for anvil
	anvilConfig := LocalEthereumAnvilChainConfig("ethereum")

	cf := interchaintest.NewBuiltinChainFactory(zaptest.NewLogger(t), []*interchaintest.ChainSpec{
		{
			ChainName:   "ethereum",
			Name:        "ethereum",
			Version:     "latest",
			ChainConfig: anvilConfig,
		},
	})

	chains, err := cf.Chains(t.Name())
	require.NoError(t, err)

	ethereumChain := chains[0].(*ethereum.EthereumChain)

	ic := interchaintest.NewInterchain().
		AddChain(ethereumChain)

	require.NoError(t, ic.Build(ctx, eRep, interchaintest.InterchainBuildOptions{
		TestName:  t.Name(),
		Client:    client,
		NetworkID: network,
		// BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),
		SkipPathCreation: true, // Skip path creation, so we can have granular control over the process
	}))
	fmt.Println("Interchain built")

	// Create and fund a user using GetAndFundTestUsers
	ethUserInitialAmount := math.NewInt(2 * ethereum.ETHER)
	users := interchaintest.GetAndFundTestUsers(t, ctx, "user", ethUserInitialAmount, ethereumChain)
	// ethUser := users[0]
	fmt.Println(users)

	// Sleep for additional testing
	time.Sleep(10 * time.Hour)

}

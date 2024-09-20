package testsuite

import (
	"context"
	"fmt"
	"os"

	dockerclient "github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/stretchr/testify/suite"
	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"

	"cosmossdk.io/math"
	logger "github.com/JackalLabs/storage-outpost/e2e/interchaintest/logger"
	interchaintest "github.com/strangelove-ventures/interchaintest/v7"
	"github.com/strangelove-ventures/interchaintest/v7/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
)

type TestSuite struct {
	suite.Suite

	ChainB       *cosmos.CosmosChain
	UserB        ibc.Wallet
	UserC        ibc.Wallet
	ChainBConnID string
	dockerClient *dockerclient.Client
	Relayer      ibc.Relayer
	network      string
	logger       *zap.Logger
	ExecRep      *testreporter.RelayerExecReporter
	PathName     string
}

// SetupSuite sets up the chains, relayer, user accounts, clients, and connections
func (s *TestSuite) SetupSuite(ctx context.Context, chainSpecs []*interchaintest.ChainSpec) {

	t := s.T()

	s.logger = zaptest.NewLogger(t)
	s.dockerClient, s.network = interchaintest.DockerSetup(t)

	cf := interchaintest.NewBuiltinChainFactory(s.logger, chainSpecs)

	chains, err := cf.Chains(t.Name())
	s.Require().NoError(err)
	s.ChainB = chains[0].(*cosmos.CosmosChain)

	s.ExecRep = testreporter.NewNopReporter().RelayerExecReporter(t)

	ic := interchaintest.NewInterchain().
		AddChain(s.ChainB)

	s.Require().NoError(ic.Build(ctx, s.ExecRep, interchaintest.InterchainBuildOptions{
		TestName:         t.Name(),
		Client:           s.dockerClient,
		NetworkID:        s.network,
		SkipPathCreation: true,
	}))
	logger.InitLogger()

	// Fund user accounts on ChainA and ChainB
	// WARNING: This number can't be too high or the faucet can't seem to have enough to fund accounts
	// Perfect number is between 10_000_000_000 and 1_000_000_000_000
	const userFunds = int64(1_000_000_000_000)
	userFundsInt := math.NewInt(userFunds)

	// this is the seed phrase for the danny user that appears in all of canine-chain's testing scripts
	userBSeed := "brief enhance flee chest rabbit matter chaos clever lady enable luggage arrange hint " +
		"quarter change float embark canoe chalk husband legal dignity music web"
	userB, err := interchaintest.GetAndFundTestUserWithMnemonic(ctx, "jkl", userBSeed, userFundsInt, s.ChainB)
	s.Require().NoError(err)

	s.UserB = userB //the jackal user

	userCSeed := "raven symbol today record infant degree glad use risk outdoor stool strike clay " +
		"tomorrow salute method mystery behave ivory repeat young hover glare essence"
	userC, err := interchaintest.GetAndFundTestUserWithMnemonic(ctx, "jkl", userCSeed, userFundsInt, s.ChainB)
	s.Require().NoError(err)

	s.UserC = userC

	// NOTE: not really sure where to pass this in atm
	usingPorts := nat.PortMap{}

	caninedConfig := s.ChainB.Config()

	if caninedConfig.HostPortOverride != nil {
		for intP, extP := range caninedConfig.HostPortOverride {
			usingPorts[nat.Port(fmt.Sprintf("%d/tcp", intP))] = []nat.PortBinding{
				{
					HostPort: fmt.Sprintf("%d", extP),
				},
			}
		}
		fmt.Printf("Port Overrides: %v. Using: %v\n", caninedConfig.HostPortOverride, usingPorts)
	}

	t.Cleanup(
		func() {
			if os.Getenv("KEEP_CONTAINERS_RUNNING") != "1" {
				err := s.Relayer.StopRelayer(ctx, s.ExecRep)
				if err != nil {
					t.Logf("an error occurred while stopping the relayer: %s", err)
				}
			} else {
				t.Logf("Skipping relayer stop due to KEEP_CONTAINERS_RUNNING flag")
			}
		},
	)
}

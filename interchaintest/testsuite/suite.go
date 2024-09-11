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
	"github.com/strangelove-ventures/interchaintest/v7/relayer"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
)

type TestSuite struct {
	suite.Suite

	ChainA       *cosmos.CosmosChain
	ChainB       *cosmos.CosmosChain
	ChainAFaucet ibc.Wallet
	UserA        ibc.Wallet
	UserA2       ibc.Wallet
	UserA3       ibc.Wallet
	UserB        ibc.Wallet
	ChainAConnID string
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
	if len(chainSpecs) != 2 {
		panic("ContractTestSuite requires exactly 2 chain specs")
	}

	t := s.T()

	s.logger = zaptest.NewLogger(t)
	s.dockerClient, s.network = interchaintest.DockerSetup(t)

	cf := interchaintest.NewBuiltinChainFactory(s.logger, chainSpecs)

	chains, err := cf.Chains(t.Name())
	s.Require().NoError(err)
	s.ChainA = chains[0].(*cosmos.CosmosChain)
	s.ChainB = chains[1].(*cosmos.CosmosChain)

	// docker run -it --rm --entrypoint echo ghcr.io/cosmos/relayer "$(id -u):$(id -g)"
	customRelayerImage := relayer.CustomDockerImage("ghcr.io/cosmos/relayer", "", "100:1000")

	s.Relayer = interchaintest.NewBuiltinRelayerFactory(
		ibc.CosmosRly,
		zaptest.NewLogger(t),
		customRelayerImage,
	).Build(t, s.dockerClient, s.network)

	s.ExecRep = testreporter.NewNopReporter().RelayerExecReporter(t)

	s.PathName = s.ChainA.Config().Name + "-" + s.ChainB.Config().Name

	ic := interchaintest.NewInterchain().
		AddChain(s.ChainA).
		AddChain(s.ChainB).
		AddRelayer(s.Relayer, "relayer").
		AddLink(interchaintest.InterchainLink{
			Chain1:  s.ChainA,
			Chain2:  s.ChainB,
			Relayer: s.Relayer,
			Path:    s.PathName,
		})

	s.Require().NoError(ic.Build(ctx, s.ExecRep, interchaintest.InterchainBuildOptions{
		TestName:         t.Name(),
		Client:           s.dockerClient,
		NetworkID:        s.network,
		SkipPathCreation: true,
	}))
	logger.InitLogger()

	// Fund user accounts on ChainA and ChainB
	const userFunds = int64(1_00_000_000_000_000)
	userFundsInt := math.NewInt(userFunds)
	// users := interchaintest.GetAndFundTestUsers(t, ctx, t.Name(), userFunds, s.ChainA, s.ChainB)
	userASeed := "fork draw talk diagram fragile online style lecture ecology lawn " +
		"dress hat modify member leg pluck leaf depend subway grit trumpet tongue crucial stumble"
	userA, err := interchaintest.GetAndFundTestUserWithMnemonic(ctx, "wasmd", userASeed, userFundsInt, s.ChainA)
	s.Require().NoError(err)

	userA2Seed := "cage father indicate hockey rapid wrist symbol apple impulse cradle sock pony foam " +
		"survey squirrel dial drum flavor mansion bicycle master dumb album soccer"
	userA2, err := interchaintest.GetAndFundTestUserWithMnemonic(ctx, "wasmd", userA2Seed, userFundsInt, s.ChainA)
	s.Require().NoError(err)

	userA3Seed := "diagram return dose exhibit better advance task dove quiz group scheme thrive crystal " +
		"veteran clog mobile story roof over display state cannon brave machine"
	userA3, err := interchaintest.GetAndFundTestUserWithMnemonic(ctx, "wasmd", userA3Seed, userFundsInt, s.ChainA)
	s.Require().NoError(err)

	// this is the seed phrase for the danny user that appears in all of canine-chain's testing scripts
	userBSeed := "brief enhance flee chest rabbit matter chaos clever lady enable luggage arrange hint " +
		"quarter change float embark canoe chalk husband legal dignity music web"
	userB, err := interchaintest.GetAndFundTestUserWithMnemonic(ctx, "jkl", userBSeed, userFundsInt, s.ChainB)
	s.Require().NoError(err)

	s.UserA = userA   // the primary wasmd user
	s.UserA2 = userA2 // the secondary wasmd user
	s.UserA3 = userA3 // the tertiary wasmd user

	s.UserB = userB //the jackal user

	// Fund the Faucet on ChainA
	ChainAFaucetSeed := "correct rate reveal jump dutch behind witness grief fiction gather fruit " +
		"choose metal property sort sail shop nice east arrow detect east scare culture"

	// Fund user accounts on ChainA and ChainB
	const chainAFaucetFunds = int64(1_000_000_000_000_000)
	chainAFaucetFundsInt := math.NewInt(chainAFaucetFunds)

	ChainAFaucet, err := interchaintest.GetAndFundTestUserWithMnemonic(ctx, "wasmd", ChainAFaucetSeed, chainAFaucetFundsInt, s.ChainA)
	s.Require().NoError(err)
	s.ChainAFaucet = ChainAFaucet

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

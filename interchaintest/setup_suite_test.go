package interchaintest

import (
	"context"
	"encoding/json"
	"testing"

	logger "github.com/JackalLabs/jackal-hyperlane/logger"
	mysuite "github.com/JackalLabs/jackal-hyperlane/testsuite"
	types "github.com/JackalLabs/jackal-hyperlane/types"
	"github.com/stretchr/testify/suite"
)

// WARNING: remember that the suite won't boot up unless this file ends with '_test'

type ContractTestSuite struct {
	mysuite.TestSuite

	IcaAddress           string
	FaucetJKLHostAddress string
}

// SetupContractAndChannel starts the chains, relayer, creates the user accounts, creates the ibc clients and connections,
// sets up the contract and does the channel handshake for the contract test suite.
func (s *ContractTestSuite) SetupContractTestSuite(ctx context.Context, encoding string) {
	// This starts the chains, relayer, creates the user accounts, and creates the ibc clients and connections.
	s.SetupSuite(ctx, chainSpecs)

	logger.InitLogger()
	// Upload and Instantiate the contract on canined:
	codeId, err := s.ChainB.StoreContract(ctx, s.UserB.KeyName(), "../artifacts/mailbox.wasm")
	s.Require().NoError(err)
	logger.LogInfo(codeId)

	admin := s.UserB.FormattedAddress()

	instantiateMsg := types.NewMailboxInstantiateMsg("hrp", admin, 2)

	// TODO: instantiate the contract
	contractAddr, err := s.ChainB.InstantiateContract(ctx, s.UserB.KeyName(), codeId, instantiateMsg, false, "--gas", "500000", "--admin", s.UserB.KeyName())
	s.Require().NoError(err)
	logger.LogInfo(contractAddr)
}

func TestWithContractTestSuite(t *testing.T) {
	suite.Run(t, new(ContractTestSuite))
}

// toJSONString returns a string representation of the given value
// by marshaling it to JSON. It panics if marshaling fails.
func toJSONString(v any) string {
	bz, err := json.Marshal(v)
	if err != nil {
		panic(err)
	}
	return string(bz)
}

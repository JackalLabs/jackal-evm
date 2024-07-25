package interchaintest

import (
	"context"
	"encoding/json"
	"testing"

	mysuite "github.com/JackalLabs/jackal-hyperlane/testsuite"
	"github.com/JackalLabs/storage-outpost/e2e/interchaintest/types"
	"github.com/stretchr/testify/suite"
)

// WARNING: remember that the suite won't boot up unless this file ends with '_test'

type ContractTestSuite struct {
	mysuite.TestSuite

	Contract              *types.IcaContract
	IcaAddress            string
	FaucetOutpostContract *types.IcaContract
	FaucetJKLHostAddress  string
}

// SetupContractAndChannel starts the chains, relayer, creates the user accounts, creates the ibc clients and connections,
// sets up the contract and does the channel handshake for the contract test suite.
func (s *ContractTestSuite) SetupContractTestSuite(ctx context.Context, encoding string) {
	// This starts the chains, relayer, creates the user accounts, and creates the ibc clients and connections.
	s.SetupSuite(ctx, chainSpecs)

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

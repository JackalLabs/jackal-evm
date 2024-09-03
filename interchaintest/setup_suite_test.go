package interchaintest

import (
	"context"
	"encoding/json"
	"testing"

	mysuite "github.com/JackalLabs/jackal-evm/testsuite"
	"github.com/stretchr/testify/suite"
)

// WARNING: remember that the suite won't boot up unless this file ends with '_test'

type ContractTestSuite struct {
	mysuite.TestSuite

	IcaAddress           string
	FaucetJKLHostAddress string
}

// WARNING: the test suite we use specifically requires 2 chains
// not worth the time right now to convert it to a 1 chain test suite

// SetupContractAndChannel starts the chains, relayer, creates the user accounts, creates the ibc clients and connections,
// sets up the contract and does the channel handshake for the contract test suite.
func (s *ContractTestSuite) SetupContractTestSuite(ctx context.Context, encoding string) {
	// This starts the chains, relayer, creates the user accounts, and creates the ibc clients and connections.
	s.SetupSuite(ctx, chainSpecs)
	// TODO:
	// put a stub execute msg in mailbox and execute it from the cosmwasm signer

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

/*
	// NOTE: Contracts can in fact be instantiated
	// Unfortunately we get the below error when quering for the tx hash:

	failed to get transaction 7AB73BD5B1ED8535761FA2CCAF3986E2ECCB72DDD634EAAF3CBEAC68F892F09C: unable to
	resolve type URL /cosmwasm.wasm.v1.MsgInstantiateContract: tx parse error [cosmos/cosmos-sdk@v0.47.10/x/auth/tx/decoder.go:42]

	I believe this is because canine-chain is running cosmos-sdk 0.45 and SL's interchaintest (ict) package only supports sdk 0.47+
	canine-chain is upgrading to 0.47 or 0.50 soon, so we can just use hard values to bypass this error for now

	No need to waste time backporting SL's ict package to support sdk 0.45 and dealing with golang dependency hell

*/

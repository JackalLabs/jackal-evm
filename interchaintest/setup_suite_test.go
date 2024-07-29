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
	// time.Sleep(10 * time.Hour)
	// s.Require().NoError(err)

	/*
		// NOTE: The mailbox is in fact being instantiated with the following address: jkl14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9scsc9nr
		// Unfortunately we get the below error when quering for the tx hash:

		failed to get transaction 7AB73BD5B1ED8535761FA2CCAF3986E2ECCB72DDD634EAAF3CBEAC68F892F09C: unable to
		resolve type URL /cosmwasm.wasm.v1.MsgInstantiateContract: tx parse error [cosmos/cosmos-sdk@v0.47.10/x/auth/tx/decoder.go:42]

		I believe this is because canine-chain is running cosmos-sdk 0.45 and SL's interchaintest (ict) package only supports sdk 0.47+
		canine-chain is upgrading to 0.47 or 0.50 soon, so we can just use hard values to bypass this error for now

		No need to waste time backporting SL's ict package to support sdk 0.45 and dealing with golang dependency hell

	*/

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

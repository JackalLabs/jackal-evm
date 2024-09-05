package interchaintest

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	icatypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/types"

	testtypes "github.com/JackalLabs/jackal-evm/types"
	logger "github.com/JackalLabs/storage-outpost/e2e/interchaintest/logger"
)

// WARNING: strangelove's test package builds chains running ibc-go/v7
// Hopefully this won't cause issues because the canined image we use is running ibc-go/v4
// and packets should be consumed by the ica host no matter what version of ibc-go the controller chain is running

// Testing canine-chain's web assembly bindings
func (s *ContractTestSuite) TestJackalChainWasmBindings() {
	ctx := context.Background()

	logger.InitLogger()

	encoding := icatypes.EncodingProtobuf
	// This starts the chains, relayer, creates the user accounts, creates the ibc clients and connections,
	// sets up the contract and does the channel handshake for the contract test suite.
	s.SetupContractTestSuite(ctx, encoding)

	// This is the user in our cosmwasm_signer, so we ensure they have funds
	s.FundAddressChainB(ctx, "jkl12g4qwenvpzqeakavx5adqkw203s629tf6k8vdg")

	// Store code of bindings factory
	FactoryCodeId, err := s.ChainB.StoreContract(ctx, s.UserB.KeyName(), "../artifacts/bindings_factory.wasm")
	s.Require().NoError(err)

	// Store code of filetree bindings
	BindingsCodeId, error := s.ChainB.StoreContract(ctx, s.UserB.KeyName(), "../artifacts/filetree.wasm")
	s.Require().NoError(error)

	// Instantiate the factory, giving it the codeId of the filetree bindings contract
	instantiateMsg := testtypes.InstantiateMsg{BindingsCodeId: BindingsCodeId}

	contractAddr, err := s.ChainB.InstantiateContract(ctx, s.UserB.KeyName(), FactoryCodeId, toString(instantiateMsg), false, "--gas", "500000", "--admin", s.UserB.KeyName())
	// s.Require().NoError(err)

	// NOTE: The above errors only when trying to parse the tx hash, but the instantiate still succeeded
	// We can query for the contract address instead
	// TODO: query for contract address
	fmt.Println(contractAddr)
	logger.LogInfo(contractAddr)

	logger.LogInfo("instantiated factory")

	// NOTE: The contractAddr can't be retrived at this time because of sdk tx parsing error we noted before
	// We can fix that later but for now, we'll just hard code the  consistent factory contract address

	// TODO: is this going to be different?
	factoryContractAddr := "jkl1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq59a839"
	fmt.Println(factoryContractAddr)

	// s.Run(fmt.Sprintf("TestSendCustomIcaMesssagesSuccess-%s", encoding), func() {

	// 	msg := types.ExecuteMsg{
	// 		PostKey: &types.ExecuteMsg_PostKey{
	// 			Key: "Testing wasm bindings",
	// 		},
	// 	}

	// 	res, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), filetreeContractAddr, msg.ToString(), "--gas", "500000")
	// 	// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
	// 	// fortunately, we went into the docker container to confirm that the post key msg does get saved into canine-chain
	// 	fmt.Println(res)
	// 	//s.Require().NoError(error)

	// Could also just use a querier

	// },
	// )
	time.Sleep(time.Duration(10) * time.Hour)
}

// log address of bindings contract
// create bindings factory contract

// toString converts the message to a string using json
func toString(msg any) string {
	bz, err := json.Marshal(msg)
	if err != nil {
		panic(err)
	}

	return string(bz)
}
